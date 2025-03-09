use crate::foliage_density::FoliageDensityMapsComponent;
use crate::foliage_density::FoliageDensityMapU8;
 
 
use crate::FoliageTypesManifest;
use bevy::prelude::*;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};






#[derive(Component)]
#[require(Transform,Visibility)]
pub struct FoliageRoot ; 


//#[derive(Resource)]
//pub struct FoliageConfigResource(pub FoliageConfig);



/// A foliage config describes the foliage of a level including the dimensions and paths to the foliage types manifest and density (binary file)
#[derive(Clone, Debug, Serialize, Deserialize, Component )]
pub struct FoliageScene {
    pub boundary_dimensions: IVec2,
    pub chunk_rows: usize,
    

    pub render_distance: Option<f32>,
    pub height_scale: f32,

   
    pub foliage_types_manifest_path: String,
     pub foliage_density_data_path: Option<String>,   //the density for all layers 

}

impl FoliageScene {
    pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }
}





pub struct LoadFoliageScene {
    pub name: String,
    pub path: String, 

}  //path 

impl Command for LoadFoliageScene {




        fn apply(self, world: &mut World) { 
               let foliage_config_path = format!("{}{}",  &self.path , &self.name  );

       
     
               let foliage_scene = FoliageScene::load_from_file(&foliage_config_path)
                    .expect("Could not load foliage config");

                let foliage_types_manifest =
                    FoliageTypesManifest::load_from_file(&foliage_scene.foliage_types_manifest_path)
                        .expect("Could not load foliage types manifest");

                let foliage_density_data_path = foliage_scene.foliage_density_data_path.clone(); 
                let foliage_definitions = foliage_types_manifest.foliage_definitions .clone(); 
                let foliage_dimensions = foliage_scene.boundary_dimensions .clone(); 

 
                let foliage_density_maps_component = 
                    FoliageDensityMapsComponent::create_or_load( 
                        foliage_density_data_path, 
                        foliage_dimensions, 
                        foliage_definitions

                     ) ;
 


                world.commands().spawn( (
                    Name::new("foliage root"),
                    FoliageRoot, 
                    foliage_scene,

                    foliage_types_manifest,
                    foliage_density_maps_component,

                   
                ) );
                 

             //   let foliage_density_data = ;

             
               
        }
}



pub struct DespawnFoliageScene  ;

impl Command for DespawnFoliageScene {
 
        fn apply(self, world: &mut World) { 

              let mut foliage_root_query = world.query_filtered::<Entity, With<FoliageRoot> >() ;

              for foliage_root_entity in foliage_root_query.iter(world).collect::<Vec<_>>() {

                  if let Some(mut cmd) = world.commands().get_entity( foliage_root_entity ){
                         cmd.despawn_recursive();
                  }

              }

        }
}

















/*

 //this is a RON file ! 
 (
   

    boundary_dimensions: (1024, 1024), // IVec2(x, y)
    chunk_rows: 8, // 8 x 8 .. 128 units each 

   
    render_distance: Some(200.0),
   
    height_scale: 0.001,
  
 
   // foliage_data_files_path:  "assets/foliage/foliage_scenes/" ,

    foliage_types_manifest_path:  "assets/foliage/foliage_manifest.ron" ,  //defines the layers 
 
    foliage_density_data: Some("assets/foliage/foliage_density_maps/world_foliage.densitymap"), 
    
)

 

*/