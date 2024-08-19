 
use bevy::{asset::load_internal_asset, prelude::*};
 
 

use std::time::Duration;
 
//use crate::chunk::TerrainMaterialExtension;
 
 
pub mod foliage;
pub mod foliage_chunk;
  
pub struct BevyFoliageToolPlugin {
    //task_update_rate: Duration,
}

impl Default for BevyFoliageToolPlugin {
    fn default() -> Self {
        Self {
           // task_update_rate: Duration::from_millis(250),
        }
    }
}
impl Plugin for BevyFoliageToolPlugin {
    fn build(&self, app: &mut App) {

        app



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
