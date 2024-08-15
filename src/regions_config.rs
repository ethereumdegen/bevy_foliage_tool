/*

this is loaded from a RON file


also should incorporate the paths to the height and splat folders for their texture handles...

*/
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Component, Deserialize, Serialize, Clone)]
pub struct RegionsConfig {
    pub boundary_dimensions: Vec2, 
 
    pub region_texture_path: PathBuf,
    pub region_color_map_texture_path: PathBuf,
    pub regions_manifest_file: PathBuf,
    
}

impl Default for RegionsConfig {
    fn default() -> Self {
        Self {
            // chunk_width: 64.0 ,
            boundary_dimensions: Vec2::new(1024.0, 1024.0), //this should match the heightmap dimensions... consider removing this var or changing how it fundamentally works .
            

            region_texture_path: "regions/regions.png".into(),
            region_color_map_texture_path: "regions/region_color_map.png".into(),
            regions_manifest_file: "regions/regions_manifest.ron".into(),
            
        }
    }
}

impl RegionsConfig {
    pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

  
}
