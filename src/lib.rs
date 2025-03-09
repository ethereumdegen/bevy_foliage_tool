use crate::foliage_assets::FoliageAssetsResource;
 
use crate::foliage_types::FoliageTypesManifest;
 
use bevy::{asset::load_internal_asset, prelude::*};
use edit::bevy_foliage_edits_plugin;
 

//use crate::chunk::TerrainMaterialExtension;

pub mod edit;
//pub mod foliage;
pub mod foliage_chunk;

pub mod foliage_scene;

pub mod foliage_chunk_layer;


//pub mod foliage_layer;
pub mod foliage_density;
pub mod foliage_types;
//pub mod foliage_loading_state;
pub mod foliage_assets;
pub mod foliage_proto;

pub mod foliage_registration;

pub mod foliage_material;
pub mod foliage_viewer;
pub mod noise;

pub struct BevyFoliageToolPlugin ;
impl Plugin for BevyFoliageToolPlugin {
    fn build(&self, app: &mut App) {
        
        app //.insert_resource(FoliageConfigResource(foliage_config))
           // .insert_resource(FoliageTypesResource(foliage_types_manifest))

              .add_plugins(foliage_assets::foliage_assets_plugin)
              
            // .add_plugins(foliage_loading_state::foliage_loading_state_plugin)
            // .add_plugins(foliage::foliage_plugin)
            .add_plugins(foliage_chunk::foliage_chunks_plugin)
            .add_plugins(noise::noise_plugin)
            .add_plugins(foliage_density::foliage_density_plugin)
            .add_plugins(foliage_chunk_layer::foliage_chunk_layer_plugin)
            .add_plugins(bevy_foliage_edits_plugin)

           

             ;
    }
}

/// This plugin is responsible for attaching models and materials to FoliageProto entities
pub struct BevyFoliageProtoPlugin;
impl Plugin for BevyFoliageProtoPlugin {
    fn build(&self, app: &mut App) {
        app
            //  .insert_resource( self.foliage_assets_resource.clone() )
           
            .add_plugins(foliage_proto::foliage_proto_plugin);
    }
}

pub struct BevyFoliageMaterialPlugin;
impl Plugin for BevyFoliageMaterialPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_plugins(foliage_material::foliage_material_plugin)
          .add_plugins(foliage_registration::foliage_registration_plugin);
          
    }
}
