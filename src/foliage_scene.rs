

 
use crate::foliage_config::FoliageConfigResource;
use crate::FoliageTypesResource;
use crate::foliage_layer::FoliageLayerData;


use std::fs::{File, create_dir_all};
use std::io::{Write, Read};
use std::path::Path;
use crate::foliage_layer::FoliageLayer;
use bevy::prelude::*;

use bevy::utils::HashMap;
use serde::{Serialize,Deserialize};


pub(crate) fn foliage_scene_plugin(app: &mut App ) {
    app
    		

    	.add_systems(Update, unpack_foliage_scene_data_components)
    	;



  }




#[derive(Component,Clone,Debug,Serialize,Deserialize)]
pub struct FoliageScene  {

	pub foliage_scene_name: String, 

	pub foliage_layer_entities_map: HashMap<usize, Entity >, 
	 
}



/// A serializeable struct that can be cached on disk as a file 
/// and it describes all of the foliage layers for an entire scene or level
/// where each foliage layer has a density map, y_offset map, mesh handle and material handle  
#[derive(Component,Clone,Debug,Serialize,Deserialize)]
pub struct FoliageSceneData {

	pub foliage_scene_name: String, 
	pub foliage_layers: HashMap< usize, FoliageLayerData >

}

/*
impl Default for FoliageScene {



	fn default() -> Self { 

		Self  { 
			foliage_layers: HashMap::new(),

		}

	  }


}
*/


impl FoliageSceneData {


	 pub fn new(
		scene_name: &str,

		) -> Self{


		Self{


			foliage_scene_name: scene_name.into(), 
			foliage_layers: HashMap::new()

		}
	} 


	/*pub fn load_or_create(  
		scene_name: &str
	) -> Self {


		//try to load from file..  if fail, then 



		Self::new( scene_name )
	}*/


/*	pub fn save_to_disk(  
		&self,
		foliage_data_files_path: &str
	)  -> bool {

		true
	}*/


	pub fn create_or_load(

		foliage_data_files_path: &str ,
		scene_name: &str
	 ) -> Self { 


	 	let load_from_disk_result = Self::load_from_disk(
			foliage_data_files_path,
			scene_name

		 );

//	 	info!("foliage load_from_disk_result {:?}",load_from_disk_result);

		match load_from_disk_result {

			Some(loaded) =>  loaded ,

			None =>  Self::new( scene_name )   
		} 
	}



	pub fn save_to_disk(
	    &self,
	    foliage_data_files_path: &str
	) -> Result<(), String> {

		let scene_name = self.foliage_scene_name.clone();
	    // Ensure the directory exists
	   let full_file_path = format!("{}{}", foliage_data_files_path, scene_name);
		    

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
	        Err(e) => {
	            Err(format!("Failed to create file: {}", e))
	        }
	    }
	}

	/*pub fn save_to_disk_debug(

		 &self,
	    foliage_data_files_path: &str

		)  -> Result<(), String>  {
		let scene_name = self.foliage_scene_name.clone();
		let full_file_path = format!("{}{}", foliage_data_files_path, scene_name);
		    
		 let ron = ron::ser::to_string(&self).unwrap();
	      let file_saved = std::fs::write(full_file_path, ron);

	      Ok(())

	}*/

	// This function loads the FoliageSceneData from disk
	pub fn load_from_disk(
		
	    foliage_data_files_path: &str,
	    scene_name: &str, 
	) -> Option<Self> {

		let full_file_path = format!("{}{}", foliage_data_files_path, scene_name);
	    
	 
	    // Open the file for reading
	    let file_result = File::open( full_file_path );

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


/// This will spawn child layer entities  -> when those get 'added', more things happen../spawn..
fn unpack_foliage_scene_data_components(

   mut commands: Commands, 

   foliage_scene_data_query: Query<(Entity, &FoliageSceneData)>,

   foliage_config_resource: Res<FoliageConfigResource>,
   foliage_types_resource: Res<FoliageTypesResource>


){

	// this is a beautiful abomination 

	for (foliage_scene_entity, foliage_scene_data) in foliage_scene_data_query.iter(){

 

      	let mut layers_data_array =  foliage_scene_data.foliage_layers.clone();
      		
      	let foliage_config = &foliage_config_resource.0;
      	let boundary_dimensions = foliage_config.boundary_dimensions; 

      	if layers_data_array.is_empty(){

      		//add in the ones from the types manifest 
      		let foliage_definitions = &foliage_types_resource.0.foliage_definitions; 
      		
      		for (foliage_def_index, _foliage_definition) in foliage_definitions.iter().enumerate() {

      			layers_data_array.insert( foliage_def_index, FoliageLayerData::new( foliage_def_index, boundary_dimensions ) );  

      		}
      	}



      	let mut foliage_layer_entities_map = HashMap::new();

 	
    

       let Some(mut foliage_scene_cmd)	= commands.get_entity(foliage_scene_entity) else {continue};

   
    		info!("unpacking foliage scene ");
      		foliage_scene_cmd
      		.remove::<FoliageSceneData>()
		      
      		.with_children( |child_builder| 
	      		{
	      			  for (layer_index,layer_data) in layers_data_array {

	      			 		 	info!("spawn foliage layer data ");

	      						   let layer_entity = child_builder.spawn( 
	      							 	(
	      									SpatialBundle::default() , 
	      									layer_data.clone()
	      								)

	      							   ).id();


	      						   foliage_layer_entities_map.insert(  layer_index , layer_entity) ;
 						}
	      		}
      		 )

      		.insert(FoliageScene {	
		      	foliage_scene_name: foliage_scene_data.foliage_scene_name.clone(),
		      	foliage_layer_entities_map

		      }) ; 

 



	}









}