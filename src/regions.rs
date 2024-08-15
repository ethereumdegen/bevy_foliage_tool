 
use bevy::asset::{AssetPath, LoadState};
use bevy::pbr::{ExtendedMaterial, OpaqueRendererMethod};
use bevy::prelude::*;
use bevy::render::render_resource::{
    TextureFormat,
};

use super::regionmap::{RegionMap,RegionMapU8,SubRegionMapU8};

use bevy::utils::HashMap;

 
use crate::regions_config::RegionsConfig;
use crate::regions_material::{RegionsMaterial, RegionsMaterialExtension, ToolPreviewUniforms};
 


#[derive(Resource, Default)]
pub struct RegionsDataMapResource {
    pub regions_data_map: Option<RegionMapU8>, // Keyed by chunk id
}



#[derive(Component)]
pub struct RegionPlaneMesh {

}

 

#[derive(Event)]
pub enum RegionDataEvent {
    RegionMapNeedsReloadFromResourceData
} 
#[derive(Default, PartialEq, Eq)]
pub enum RegionsDataStatus {
    //us this for texture image and splat image and alpha mask .. ?
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Component, Default)]
pub struct  RegionsData {
     
    pub regions_data_status: RegionsDataStatus,

    texture_image_handle: Option<Handle<Image>>,
    color_map_texture_handle:  Option<Handle<Image>>,
 
    regions_image_data_load_status: bool ,
 
}

impl RegionsData {
    pub fn new() -> Self {
        let regions_data = RegionsData::default();

         
        regions_data
    }
}



pub type PlanarPbrBundle = MaterialMeshBundle<RegionsMaterialExtension>;


pub fn initialize_regions(
    mut commands: Commands,

    mut asset_server: ResMut<AssetServer>, 

    mut regions_query: Query<(Entity, &mut RegionsData, &RegionsConfig)>,

    mut meshes: ResMut <Assets<Mesh>>,
    mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,

    mut images: ResMut<Assets<Image>>
) {
    for (region_entity, mut regions_data, regions_config) in regions_query.iter_mut() {
        if regions_data.regions_data_status ==  RegionsDataStatus::NotLoaded {
                

         


             if regions_data.color_map_texture_handle.is_none() {
                 regions_data.color_map_texture_handle = Some( 
                    asset_server.load( 
                        regions_config.region_color_map_texture_path.clone() 
                     ) );

           }


             if regions_data.regions_image_data_load_status == false {continue};



             let regions_texture = regions_data.get_regions_texture_image().clone();


             let regions_material: Handle<RegionsMaterialExtension> =
                region_materials.add(ExtendedMaterial {
                    base: StandardMaterial {
                        // can be used in forward or deferred mode.
                        opaque_render_method: OpaqueRendererMethod::Auto,
                        alpha_mode: AlphaMode::Blend,

                        base_color: Color::rgba(1.0, 1.0, 1.0, 0.1),

                        reflectance: 0.0,
                        perceptual_roughness: 0.9,
                        specular_transmission: 0.1,

                       unlit:true, 
                      fog_enabled :false,

                        
                        ..Default::default()
                    },
                    extension: RegionsMaterial {
                         
                        tool_preview_uniforms: ToolPreviewUniforms::default(),
                        regions_texture: regions_texture.clone(),
                        color_map_texture: regions_data.color_map_texture_handle.clone() ,
                      
                        ..default()
                    },
                });

           let dimensions = regions_config.boundary_dimensions.clone();

             // ground plane
           let regions_plane = commands.spawn(PlanarPbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size( dimensions.x, dimensions.y )),
                material: regions_material,
                transform: Transform::from_xyz(dimensions.x/2.0, 0.0, dimensions.y/2.0),
                ..default()
            })
           .insert(RegionPlaneMesh{})
           .id();

            commands.entity(  region_entity  ).add_child(  regions_plane ) ;



            //do regionmap load_from_file .. ? 

            regions_data.regions_data_status = RegionsDataStatus::Loaded
        }
    }
}

impl RegionsData {
    pub fn get_regions_texture_image(&self) -> &Option<Handle<Image>> {
        &self.texture_image_handle
    }

      
 
}


pub fn load_regions_texture_from_image(
    mut regions_query: Query<(&mut RegionsData, &RegionsConfig)>,

    mut regions_data_res: ResMut<RegionsDataMapResource>,

    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    for (mut regions_data, regions_config) in regions_query.iter_mut() {
        if regions_data.texture_image_handle.is_none() {
            let texture_path = &regions_config.region_texture_path;
            let tex_image = asset_server.load(AssetPath::from_path(texture_path));
            regions_data.texture_image_handle = Some(tex_image);
        }

        if regions_data.regions_image_data_load_status ==false {
            let texture_image: &mut Image = match &regions_data.texture_image_handle {
                Some(texture_image_handle) => {
                    let texture_image_loaded = asset_server.get_load_state(texture_image_handle);

                    if texture_image_loaded != Some(LoadState::Loaded) {
                      //  println!("regions texture not yet loaded");
                        continue;
                    }

                    images.get_mut(texture_image_handle).unwrap()
                }
                None => continue,
            };

            let raw_data = RegionMapU8::load_from_image(texture_image).ok().unwrap();

            regions_data_res.regions_data_map = Some( *raw_data  ) ;

            // Specify the desired texture format
            let desired_format = TextureFormat::Rgba8Uint;


            texture_image.texture_descriptor.format = desired_format; 
            // Create a new texture descriptor with the desired format
           // let mut texture_descriptor = TextureDescriptor

             

            regions_data.regions_image_data_load_status =true;
        }
    }
}


 


pub fn listen_for_region_events(
    mut commands : Commands, 
   mut  evt_reader: EventReader<RegionDataEvent>,

   regions_data_res: Res <RegionsDataMapResource>,
  mut region_data_query: Query<(&mut RegionsData, &RegionsConfig)> , 


  //   asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    //mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,


    //plane_mesh_query: Query<Entity, With<RegionPlaneMesh>>,

     plane_mat_ext_handle_query: Query<&Handle<RegionsMaterialExtension>, With<RegionPlaneMesh>>,

    mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,

    ){

    for evt in evt_reader.read(){


          let Some((mut region_data, _region_config)) = region_data_query
                    .get_single_mut().ok() else {continue};


       


        match evt{
            RegionDataEvent::RegionMapNeedsReloadFromResourceData =>  {

 

                let data_in_resource = &regions_data_res.regions_data_map;

                if let Some(data_map ) = data_in_resource {

                    let data_map_vec : RegionMapU8 = data_map.to_vec();
                    let new_regions_texture = data_map_vec.to_image();


                     region_data.texture_image_handle = Some(images.add(new_regions_texture));

                     info!("update texture image handle ");


                      let Some(mat_ext_handle) = plane_mat_ext_handle_query.get_single().ok() else {continue};

                      let Some(   mat_ext )  = region_materials.get_mut(mat_ext_handle) else {continue} ;

                      mat_ext.extension.regions_texture = region_data.texture_image_handle.clone();

 


                }
 


            },
        }




    }

}



