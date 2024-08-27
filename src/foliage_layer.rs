use bevy::prelude::*;

use bevy::utils::HashMap;
use serde::{Serialize,Deserialize};


#[derive(Component,Clone,Debug,Serialize,Deserialize)]
pub struct FoliageLayer {
	pub foliage_index: usize, //refer to the config 

	pub density_map: FoliageDensityMapU8,
	pub base_height_map: FoliageBaseHeightMapU8, 

	 
}






#[derive(Component,Debug,Clone,Serialize,Deserialize)]
pub struct FoliageBaseHeightMapU8(Vec<Vec<u8>>);




#[derive(Component,Debug,Clone,Serialize,Deserialize)]
pub struct FoliageDensityMapU8(Vec<Vec<u8>>);

