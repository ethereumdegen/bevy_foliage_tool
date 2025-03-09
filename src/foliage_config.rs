use crate::foliage_density::FoliageDensityResource;
use crate::FoliageTypesResource;
use crate::FoliageTypesManifest;
use bevy::prelude::*;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct FoliageConfigResource(pub FoliageConfig);



/// A foliage config describes the foliage of a level including the dimensions and paths to the foliage types manifest and density (binary file)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoliageConfig {
    pub boundary_dimensions: IVec2,
    pub chunk_rows: usize,
    

    pub render_distance: Option<f32>,
    pub height_scale: f32,

   
    pub foliage_types_manifest_path: String,
     pub foliage_density_data_path: Option<String>,   //the density for all layers 

}

impl FoliageConfig {
    pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }
}





pub struct LoadFoliageConfig {
    pub name: String,
    pub path: String, 

}  //path 

impl Command for LoadFoliageConfig {




        fn apply(self, world: &mut World) { 
               let foliage_config_path = format!("{}{}",  &self.path , &self.name  );

       
     
               let foliage_config = FoliageConfig::load_from_file(&foliage_config_path)
                    .expect("Could not load foliage config");

                let foliage_types_manifest =
                    FoliageTypesManifest::load_from_file(&foliage_config.foliage_types_manifest_path)
                        .expect("Could not load foliage types manifest");

                let foliage_density_data_path = foliage_config.foliage_density_data_path.clone(); 
                let foliage_definitions = foliage_types_manifest.foliage_definitions .clone(); 
                let foliage_dimensions = foliage_config.boundary_dimensions .clone(); 


                //happens instantly i believe .. ? 
                 world.insert_resource( FoliageConfigResource (foliage_config)  );
                 world.insert_resource( FoliageTypesResource( foliage_types_manifest ) );


               let foliage_density_resource  = match  foliage_density_data_path {

                    Some(ref full_file_path) => {


                           match FoliageDensityResource::load_from_disk( full_file_path ) {

                            Some(r) => r,
                               None => {
                                    warn!( "unable to load foliage density file! making a new one . " );
                                        FoliageDensityResource::new ( foliage_dimensions,  foliage_definitions   )  
                                }
                            }

                          
                    }

                    None => {
                           FoliageDensityResource::new ( foliage_dimensions,  foliage_definitions   )  

                    }

                };


               
                 world.insert_resource(  foliage_density_resource  );
                 

             //   let foliage_density_data = ;

             
               
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