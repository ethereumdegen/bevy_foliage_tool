use crate::tool_preview::update_tool_uniforms;
use crate::regions::{listen_for_region_events, RegionDataEvent, RegionsDataMapResource};
use crate::edit::BevyRegionEditsPlugin;
use crate::regions::load_regions_texture_from_image;
use crate::regions_material::RegionsMaterialExtension;
use bevy::time::common_conditions::on_timer;
use bevy::{asset::load_internal_asset, prelude::*};
 
use regions::{ initialize_regions,  };

use std::time::Duration;
 
//use crate::chunk::TerrainMaterialExtension;
use crate::regions_material::{RegionsMaterial,REGION_SHADER_HANDLE};
 
 
 
 pub mod edit;



pub mod regionmap;
 
pub mod regions;
pub mod regions_config;
 
pub mod regions_material;
pub mod tool_preview;

pub struct BevyRegionsPlugin {
    task_update_rate: Duration,
}

impl Default for BevyRegionsPlugin {
    fn default() -> Self {
        Self {
            task_update_rate: Duration::from_millis(250),
        }
    }
}
impl Plugin for BevyRegionsPlugin {
    fn build(&self, app: &mut App) {
        // load terrain shader into cache
        load_internal_asset!(
            app,
            REGION_SHADER_HANDLE,
            "shaders/regions.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<RegionsMaterialExtension>::default());


        app.add_plugins( BevyRegionEditsPlugin::default() ) ;
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
        );
        
        
 
    

        
    }
}
