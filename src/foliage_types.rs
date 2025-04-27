use bevy::prelude::*;
use bevy::platform::collections::hash_map::HashMap ;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

//#[derive(Resource)]
//pub struct FoliageTypesResource(pub FoliageTypesManifest);

#[derive(Clone, Debug, Serialize, Deserialize, Component )]
pub struct FoliageTypesManifest {
    pub foliage_definitions: Vec<FoliageDefinition>,
    pub foliage_mesh_definitions: HashMap<String,String> ,
    pub foliage_material_definitions: HashMap< String, FoliageMaterialDefinition >,
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

#[derive(Clone, Debug, Serialize, Deserialize, Reflect )]
pub struct FoliageDefinition {
    pub name: String,

    pub mesh_name: Option<String>,
    pub material_name: Option<String>,
}


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct FoliageMaterialDefinition {

    #[serde(default)]
    pub material_preset: FoliageMaterialPreset, 
    
    pub base_color: Option<Srgba>,
    pub base_color_texture: Option<String>, 

}

#[derive(Clone,Debug,Serialize,Deserialize,Default)]
pub enum FoliageMaterialPreset{
    #[default]
    Standard,
    Foliage 
}