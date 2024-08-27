

 
use crate::foliage_layer::FoliageLayer;
use bevy::prelude::*;

use bevy::utils::HashMap;
use serde::{Serialize,Deserialize};


/// A serializeable struct that can be cached on disk as a file 
/// and it describes all of the foliage layers for an entire scene or level
/// where each foliage layer has a density map, y_offset map, mesh handle and material handle  
#[derive(Component,Clone,Debug,Serialize,Deserialize)]
pub struct FoliageScene {

	pub foliage_layers: HashMap< usize, FoliageLayer >

}

impl Default for FoliageScene {



	fn default() -> Self { 

		Self  { 
			foliage_layers: HashMap::new(),

		}

	  }


}



impl FoliageScene {





	pub fn load_or_create(  
		scene_name: &str
	) -> Self {


		//try to load from file..  if fail, then 



		FoliageScene::default()
	}


	pub fn save_to_disk(  
		&self,
		scene_name: &str
	)  -> bool {

		true
	}


}