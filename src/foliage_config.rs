use bevy::prelude::*;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Serialize,Deserialize};


pub(crate) fn foliage_config_plugin(app: &mut App ) {
    app
    	
    	;



  }



#[derive(Resource)]
pub struct FoliageConfigResource( pub FoliageConfig  );


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct FoliageConfig{

	boundary_dimensions: IVec2,
	chunk_rows: usize,
	foliage_data_files_path: Option<String>,
	foliage_types_manifest_path: Option<String>, 

  
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