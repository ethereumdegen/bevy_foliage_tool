 
use crate::foliage_config::FoliageConfig;
use bevy::{prelude::*};
use foliage_config::FoliageConfigResource;
 
 

use std::time::Duration;
 
//use crate::chunk::TerrainMaterialExtension;
 
 
pub mod foliage;
pub mod foliage_chunk;

pub mod foliage_scene;
pub mod foliage_layer;
pub mod foliage_config;
pub mod foliage_loading_state;
  
pub struct BevyFoliageToolPlugin {
    foliage_config_path: String 
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

        app

        .insert_resource(FoliageConfigResource(
            FoliageConfig::load_from_file(&self.foliage_config_path)
            .expect("Could not load foliage config")
         ))

        .add_plugins(foliage_loading_state::foliage_loading_state_plugin)

        .add_plugins(foliage::foliage_plugin)

        .add_plugins(foliage_chunk::foliage_chunks_plugin)



        ;


        // load terrain shader into cache
        /*load_internal_asset!(
            app,
            REGION_SHADER_HANDLE,
            "shaders/regions.wgsl",
            Shader::from_wgsl
        );*/
       // app.add_plugins(MaterialPlugin::<RegionsMaterialExtension>::default());


       /* app.add_plugins( BevyRegionEditsPlugin::default() ) ;
        app.add_event::<RegionDataEvent>() ;
        app.init_resource::<tool_preview::ToolPreviewResource>();
        app.init_resource::<RegionsDataMapResource>();
 
        app.add_systems(
            Update,
            (
                initialize_regions,
                listen_for_region_events ,
                load_regions_texture_from_image ,
                update_tool_uniforms
                ) ,
        );*/
        
        
 
    

        
    }
}
