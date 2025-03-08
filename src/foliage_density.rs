use crate::foliage_chunk::ForceRebuildFoliageChunk;
use crate::foliage_chunk::FoliageChunk;
use crate::foliage_config::FoliageConfigResource;
//use crate::foliage_layer::{FoliageDensityMapU8, FoliageLayerData};
use crate::FoliageTypesResource;

//use crate::foliage_layer::FoliageLayer;
use bevy::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub(crate) fn foliage_density_plugin(app: &mut App) {
    app
      //  .init_resource::<FoliageSceneData>()

     .add_systems(Update, handle_foliage_density_resource_changed .run_if( resource_exists_and_changed::<FoliageDensityResource> ))


   ;
}


#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FoliageDensityMapU8(pub Vec<Vec<u8>>);

impl FoliageDensityMapU8 {
    pub fn new(dimensions: IVec2) -> Self {
        let (width, height) = (dimensions.x as usize, dimensions.y as usize);
        let map = vec![vec![0u8; width]; height];
        Self(map)
    }
}



 /*
#[derive(Component, Clone, Debug)]
pub struct FoliageScene {
    //pub foliage_scene_name: String,

    pub foliage_layer_entities_map: HashMap<usize, Entity>,
}
 
*/

//contains a  foliage density map for each layer 
#[derive(Clone,Debug,Resource,Serialize,Deserialize)]
pub struct FoliageDensityResource  ( pub  HashMap<usize,FoliageDensityMapU8 >) ; 

/*{
  //  pub foliage_scene_name: String,
    pub foliage_layers: HashMap<usize,FoliageDensityMapU8 >,
}
*/


/// A serializeable struct that can be cached on disk as a file
/// and it describes all of the foliage layers for an entire scene or level
/// where each foliage layer has a density map, y_offset map, mesh handle and material handle  
/* #[derive(Component, Clone, Debug, Serialize, Deserialize, Default )]
pub struct FoliageSceneData {
  //  pub foliage_scene_name: String,
    pub foliage_layers: HashMap<usize,FoliageDensityMapU8 >,
} */

impl FoliageDensityResource {
    pub fn new( ) -> Self {
        Self  ( HashMap::new() )
    }

      fn create_or_load(foliage_data_files_path: &str, scene_name: &str) -> Self {

        let full_file_path = format!("{}{}", foliage_data_files_path , scene_name);

        let load_from_disk_result = Self::load_from_disk( &full_file_path  );

        //	 	info!("foliage load_from_disk_result {:?}",load_from_disk_result);

        match load_from_disk_result {
            Some(loaded) => loaded,

            None => Self::new( ),
        }
    }

      fn save_to_disk(&self, full_file_path: &str) -> Result<(), String> {
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

    // This function loads the FoliageSceneData from disk
      fn load_from_disk(full_file_path: &str ) -> Option<Self> {
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



pub struct SaveFoliageDensityMap {

    pub path:String 
}  //path 

impl Command for SaveFoliageDensityMap {



        fn apply(self, world: &mut World) { 

            let foliage_density_resource = world.get_resource::<FoliageDensityResource>();

         //    let mut foliage_scene_query = world.query::< ( &Name, & FoliageSceneData  ) >();


             if let Some( foliage_density_resource ) = foliage_density_resource {

                foliage_density_resource.save_to_disk( &self.path ) ;

             }
             
 
        }

}



    // changes like from painting the density ! ! 
fn handle_foliage_density_resource_changed (
    mut commands: Commands,

    foliage_density_resource: Res<FoliageDensityResource>,

   // foliage_scene_data_query: Query<(Entity, &  FoliageSceneData), Changed<FoliageSceneData>>,

    foliage_chunk_query: Query< (  Entity, & FoliageChunk  ) >

   // foliage_config_resource: Res<FoliageConfigResource>,
    //foliage_types_resource: Res<FoliageTypesResource>,
) {

    
   // if foliage_density_resource.is_changed() == false {return};

  //  for (_foliage_scene_entity, _foliage_scene) in foliage_scene_data_query.iter() {


        for (chunk_entity, _chunk) in foliage_chunk_query.iter(){ 

            if let Some(mut cmd) = commands.get_entity( chunk_entity ){

                cmd.insert( ForceRebuildFoliageChunk  );
            }
        }

   // }



}


// This will spawn child layer entities  -> when those get 'added', more things happen../spawn..
/*
fn unpack_foliage_scene_data_components(
    mut commands: Commands,

    foliage_scene_data_query: Query<(Entity, &mut FoliageSceneData), Added<FoliageSceneData>>,

    foliage_config_resource: Res<FoliageConfigResource>,
    foliage_types_resource: Res<FoliageTypesResource>,
) {
    for (foliage_scene_entity, foliage_scene_data) in foliage_scene_data_query.iter() {
        let mut layers_data_array = foliage_scene_data.foliage_layers.clone();

        let foliage_config = &foliage_config_resource.0;
        let boundary_dimensions = foliage_config.boundary_dimensions;

        //this has an issue ..
        //if layers_data_array.is_empty() {
        //add in the ones from the types manifest
        let foliage_definitions = &foliage_types_resource.0.foliage_definitions;

        for (foliage_def_index, _foliage_definition) in foliage_definitions.iter().enumerate() {
            if foliage_def_index >= layers_data_array.len() {
                layers_data_array.insert(
                    foliage_def_index,
                    FoliageLayerData::new(foliage_def_index, boundary_dimensions),
                );
            }
        }
        //  }

        let mut foliage_layer_entities_map = HashMap::new();

        let Some(mut foliage_scene_cmd) = commands.get_entity(foliage_scene_entity) else {
            continue;
        };

        info!("unpacking foliage scene ");
        foliage_scene_cmd
            .remove::<FoliageSceneData>()
            .with_children(|child_builder| {
                for (layer_index, layer_data) in layers_data_array {
                    info!("spawn foliage layer data {}", layer_index);

                    let layer_entity = child_builder
                        .spawn((
                           // Transform::default(),
                          //  Visibility::default(),
                            layer_data.clone(),
                        ))
                        .id();

                    foliage_layer_entities_map.insert(layer_index, layer_entity);
                }
            })
            .insert((

                
                FoliageScene {
             //   foliage_scene_name: foliage_scene_data.foliage_scene_name.clone(),
                foliage_layer_entities_map,
            },
            Visibility::default())  );
    }
}
*/