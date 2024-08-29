use bevy::prelude::*;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct FoliageTypesResource(pub FoliageTypesManifest);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoliageTypesManifest {
    pub foliage_definitions: Vec<FoliageDefinition>,
}

impl FoliageTypesManifest {
    pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoliageDefinition {
    pub name: String,

    pub mesh_name: Option<String>,
    pub material_name: Option<String>,
}
