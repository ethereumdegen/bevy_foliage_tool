use crate::foliage_scene::FoliageRoot;
use crate::foliage_scene::FoliageScene;
use crate::foliage_types::FoliageMaterialPreset;
 
use crate::foliage_material::FoliageMaterialExtension;
use crate::foliage_assets::FoliageMaterialHandle;
use crate::foliage_types::FoliageTypesManifest;
use bevy::prelude::*;


use crate::foliage_assets::FoliageAssetsResource;
use crate::foliage_assets::FoliageAssetsState;

pub(crate) fn foliage_registration_plugin(app: &mut App) {
    app


    .add_systems(
        Update,
        (
           register_foliage_assets.run_if( any_with_component::<FoliageTypesManifest>  )
        )
            .chain() 
            
    );
}


 


// use manifest to  do this automatically in-crate 
fn register_foliage_assets(
    asset_server: Res<AssetServer>,

  
    foliage_root_query: Query< (  &FoliageRoot,&FoliageScene,  &  FoliageTypesManifest ) , Added< FoliageTypesManifest >  >,



    mut assets_resource: ResMut<FoliageAssetsResource>,

    mut next_state: ResMut<NextState<FoliageAssetsState>>,
) {



      let Ok( (foliage_root, foliage_scene,  foliage_types ) ) = foliage_root_query.get_single () else {
           
            return  ; 
        };



	let foliage_material_definitions_array = &foliage_types.foliage_material_definitions ;


	for (mat_name,  mat_def) in foliage_material_definitions_array {



		let base_color = mat_def.base_color.unwrap_or( Srgba::WHITE ).into();

		let base_material_path = &mat_def.base_color_texture;

		let base_color_texture_handle_opt = base_material_path.as_ref().map( |p| asset_server.load(p)  );

		let standard_material = StandardMaterial {

            base_color , // not needed ?
             base_color_texture:  base_color_texture_handle_opt ,
            //double_sided: true ,
            cull_mode: None,
             unlit: true,
            double_sided: true,
           // depth_bias
           alpha_mode: AlphaMode::Mask(0.2), 


            ..default()
        } ;

        info!("register foliage material {}", mat_name );

		match mat_def.material_preset {

			FoliageMaterialPreset::Standard => {

				 assets_resource.register_foliage_material(
			        mat_name,
			        FoliageMaterialHandle::Standard(asset_server.add( standard_material )),
			    );

			}

			FoliageMaterialPreset::Foliage => {


				   assets_resource.register_foliage_material(
			       mat_name,
			        FoliageMaterialHandle::Extended(asset_server.add( FoliageMaterialExtension {  
			        	base : standard_material,
			        	..default() 
			         }  )),


			    );


			}

		}




	}


	for (mesh_name, mesh_path) in &foliage_types.foliage_mesh_definitions {


        info!("register foliage mesh {}", mesh_name );

      assets_resource.register_foliage_mesh( mesh_name , asset_server.load( mesh_path  ));

  //  assets_resource.register_foliage_mesh("grass2", asset_server.load("foliage/meshes/grass2.obj"));



	}

        //get these from the manifest 
  /*  let base_color =  Color::srgb(0.13, 0.37, 0.11).into();
     let base_color_texture =  asset_server.load(  "foliage/textures/shaded/sprite_0056.png"  );

 
    let foliage_material = FoliageMaterialExtension {
        base: StandardMaterial {

            base_color , // not needed ?
             base_color_texture: Some( base_color_texture ) ,
            //double_sided: true ,
            cull_mode: None,
             unlit: true,
            double_sided: true,
           // alpha_mode: AlphaMode,


            ..default()
        },

        ..default()
    };

    let mut green_material: StandardMaterial = Color::srgb(0.13, 0.37, 0.11).into();
    green_material.unlit = true;
    green_material.double_sided = true;

    */

    //ideally, normals will point UP

  
  /*  assets_resource.register_foliage_material(
        "standard_green",
        FoliageMaterialHandle::Standard(asset_server.add(green_material)),
    );
    assets_resource.register_foliage_material(
        "foliage_green",
        FoliageMaterialHandle::Extended(asset_server.add(foliage_material)),
    );*/

    next_state.set(FoliageAssetsState::Loaded);
}
