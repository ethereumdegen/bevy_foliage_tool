use bevy::prelude::*;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Serialize,Deserialize};





#[derive(Resource)]
pub struct FoliageConfigResource( pub FoliageConfig  );


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct FoliageConfig{

	pub boundary_dimensions: IVec2,
	pub chunk_rows: usize,
	pub foliage_data_files_path:  String ,
	pub foliage_types_manifest_path: String , 

    pub render_distance: f32,
    pub height_scale: f32,  
 
  
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