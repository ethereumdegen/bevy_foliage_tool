use crate::foliage_chunk::ForceRebuildFoliageChunk;
use crate::foliage_chunk::FoliageChunk;
 
use crate::foliage_types::FoliageDefinition;
//use crate::foliage_layer::{FoliageDensityMapU8, FoliageLayerData};
 

//use crate::foliage_layer::FoliageLayer;
use bevy::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

use  bevy::platform::collections::hash_map::HashMap;
use serde::{Deserialize, Serialize};

pub(crate) fn foliage_density_plugin(app: &mut App) {
    app
  


     .add_systems(Update, handle_foliage_density_resource_changed  )


   ;
}


#[derive(Component, Debug, Clone, Serialize, Deserialize,Default )]
pub struct FoliageDensityMapU8(pub Vec<Vec<u8>>);

impl FoliageDensityMapU8 {
    pub fn new(dimensions: IVec2) -> Self {
        let (width, height) = (dimensions.x as usize, dimensions.y as usize);
        let map = vec![vec![0u8; width]; height];
        Self(map)
    }


    pub fn get_sub_section_by_chunk_id( &self, chunk_id: u32, chunk_rows: u32, chunk_dimensions: IVec2 ) -> Self  {

        let chunk_x = (chunk_id % chunk_rows) as usize;
          let chunk_y = (chunk_id / chunk_rows) as usize;
    


         let start_x = chunk_x * chunk_dimensions.x as usize;
        let start_y = chunk_y * chunk_dimensions.y as usize;
        
        // Create a new empty density map with the chunk dimensions
        let mut subsection = FoliageDensityMapU8::new(chunk_dimensions);
        
        // Fill the subsection with data from the main density map
        for y in 0..chunk_dimensions.y as usize {
            for x in 0..chunk_dimensions.x as usize {
                // Calculate indices in the original map
                let orig_y = start_y + y;
                let orig_x = start_x + x;
                
                // Bounds checking to prevent indexing out of bounds
                if orig_y < self.0.len() && orig_x < self.0[0].len() {
                    // Copy the value from the original map to the subsection
                    subsection.0[y][x] = self.0[orig_y][orig_x];
                }
                // If out of bounds, leave as default 0 value
            }
        }
        
        subsection





    }

}



 
//contains a  foliage density map for each layer 
//#[derive(Clone,Debug,Resource,Serialize,Deserialize)]
//pub struct FoliageDensityResource  ( pub  HashMap<usize,FoliageDensityMapU8 >) ; 
 #[derive(Clone,Debug,Component,Serialize,Deserialize)]
 pub struct FoliageDensityMapsComponent( pub HashMap <  usize , FoliageDensityMapU8 > );
 

impl FoliageDensityMapsComponent {
    pub fn new( layer_dimension: IVec2,  foliage_definitions: Vec<FoliageDefinition> ) -> Self {
      

        let mut new_hashmap = HashMap::with_hasher(Default::default());


        for (layer_index, _def) in foliage_definitions.iter().enumerate() {


                new_hashmap.insert(  layer_index,  FoliageDensityMapU8::new( layer_dimension ) );
        }


        Self  ( new_hashmap )
    }
   

     pub fn save_to_disk(&self, full_file_path: &str) -> Result<(), String> {
       // let scene_name = self.foliage_scene_name.clone();
        // Ensure the directory exists
        //let full_file_path = format!("{}", foliage_data_files_path );

        // Open the file for writing
        let file_result = File::create(full_file_path);

        match file_result {
            Ok(mut file) => {
                // Serialize the data to binary using bincode
                let encoded: Vec<u8> = match bincode::serialize(self) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("Failed to serialize data: {}", e));
                    }
                };

                // Write the binary data to the file
                if let Err(e) = file.write_all(&encoded) {
                    return Err(format!("Failed to write data to file: {}", e));
                }

                Ok(())
            }
            Err(e) => Err(format!("Failed to create file: {}", e)),
        }
    }   


     pub fn create_or_load(  full_file_path:  Option<String> ,foliage_dimensions: IVec2, foliage_definitions: Vec<FoliageDefinition> ) -> Self {


              match  full_file_path {

                    Some(ref full_file_path) => {


                           match FoliageDensityMapsComponent::load_from_disk( full_file_path ) {

                            Some(r) => r,
                               None => {
                                    warn!( "unable to load foliage density file! making a new one . " );
                                        FoliageDensityMapsComponent::new ( foliage_dimensions,  foliage_definitions   )  
                                }
                            }

                          
                    }

                    None => {
                           FoliageDensityMapsComponent::new ( foliage_dimensions,  foliage_definitions   )  

                    }

                } 



    }

    // This function loads the FoliageSceneData from disk
      pub fn load_from_disk(full_file_path: &str ) -> Option<Self> {
      //  let full_file_path = format!("{}{}", foliage_data_files_path, scene_name);

        // Open the file for reading
        let file_result = File::open(full_file_path);

        match file_result {
            Ok(mut file) => {
                let mut buffer = Vec::new();

                // Read the binary data from the file
                if let Err(e) = file.read_to_end(&mut buffer) {
                    eprintln!("Failed to read data from file: {}", e);
                    return None;
                }

                // Deserialize the binary data into FoliageSceneData
                match bincode::deserialize(&buffer) {
                    Ok(data) => Some(data),
                    Err(e) => {
                        eprintln!("Failed to deserialize data: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
                None
            }
        }
    }
}


/*
pub struct CreateOrLoadFoliageDensityMap {
    pub name: String,
    pub path: String, 

}  //path 

impl Command for CreateOrLoadFoliageDensityMap {




        fn apply(self, world: &mut World) { 
         //  let full_file_path = format!("{}{}",  &self.path , &self.name  );

            let foliage_density_resource = FoliageDensityResource::create_or_load(  &self.path , &self.name ) ;
 
                world.insert_resource( foliage_density_resource );

               /*  world.spawn( (  
                    Name::new( self.name.clone() ),
                    foliage_scene_data
                     ) ); */
             


        }
}
*/

/*
pub struct SaveFoliageDensityMap {

    pub path:String 
}  //path 

impl Command for SaveFoliageDensityMap {



        fn apply(self, world: &mut World) { 

          //  let foliage_density_resource = world.get_resource::<FoliageDensityResource>();

         //    let mut foliage_scene_query = world.query::< ( &Name, & FoliageSceneData  ) >();


             if let Some( foliage_density_resource ) = foliage_density_resource {

                foliage_density_resource.save_to_disk( &self.path ) ;

             }
             
 
        }

}
*/


    // changes like from painting the density ! ! 
fn handle_foliage_density_resource_changed (
    mut commands: Commands,

   // foliage_density_resource: Res<FoliageDensityResource>,

   foliage_density_maps_query: Query < (&FoliageDensityMapsComponent ) , Changed<FoliageDensityMapsComponent>  >,

  
    foliage_chunk_query: Query< (  Entity, & FoliageChunk  ) >
 
) {

    for  foliage_density_map in foliage_density_maps_query.iter() {
   
        for (chunk_entity, _chunk) in foliage_chunk_query.iter(){ 

            if let Some(mut cmd) = commands.get_entity( chunk_entity ).ok() {

                cmd.insert( ForceRebuildFoliageChunk  );
            }
        }

   
   }

}

 