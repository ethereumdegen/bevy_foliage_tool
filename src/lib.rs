 
use crate::foliage_types::FoliageTypesResource;
use crate::foliage_types::FoliageTypesManifest;
use crate::foliage_config::FoliageConfig;
use bevy::{prelude::*};
use edit::bevy_foliage_edits_plugin;
use foliage_config::FoliageConfigResource;
 
 

use std::time::Duration;
 
//use crate::chunk::TerrainMaterialExtension;
 
 pub mod edit; 
pub mod foliage;
pub mod foliage_chunk;

pub mod foliage_scene;
pub mod foliage_layer;
pub mod foliage_config;
pub mod foliage_types;
pub mod foliage_loading_state;
  
pub struct BevyFoliageToolPlugin {
    pub foliage_config_path: String 
} 
/*
impl Default for BevyFoliageToolPlugin {
    fn default() -> Self {
        Self {
            foliage_config_path: String ,
           // task_update_rate: Duration::from_millis(250),
        }
    }
}*/
impl Plugin for BevyFoliageToolPlugin {
    fn build(&self, app: &mut App) {


        let foliage_config = FoliageConfig::load_from_file(&self.foliage_config_path)
            .expect("Could not load foliage config");


        let foliage_types_manifest = FoliageTypesManifest::load_from_file( &foliage_config.foliage_types_manifest_path )
            .expect("Could not load foliage types manifest");
       


        app  
        .insert_resource(FoliageConfigResource(
            foliage_config
         ))

         .insert_resource(FoliageTypesResource(
           foliage_types_manifest
         ))

        .add_plugins(foliage_loading_state::foliage_loading_state_plugin)

        .add_plugins(foliage::foliage_plugin)

        .add_plugins(foliage_chunk::foliage_chunks_plugin)

        .add_plugins(foliage_scene::foliage_scene_plugin)
        .add_plugins(foliage_layer::foliage_layer_plugin)


        .add_plugins(bevy_foliage_edits_plugin)
        ;


       
        
 
    

        
    }
}
