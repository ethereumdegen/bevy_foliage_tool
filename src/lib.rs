 
use crate::foliage_assets::FoliageAssetsResource;
use crate::foliage_types::FoliageTypesResource;
use crate::foliage_types::FoliageTypesManifest;
use crate::foliage_config::FoliageConfig;
use bevy::render::texture::ImageLoader;
use bevy::{asset::load_internal_asset, prelude::*};
use edit::bevy_foliage_edits_plugin;
use foliage_config::FoliageConfigResource;
 


 
//use crate::chunk::TerrainMaterialExtension;
 
 pub mod edit; 
//pub mod foliage;
pub mod foliage_chunk;

pub mod foliage_scene;
pub mod foliage_layer;
pub mod foliage_config;
pub mod foliage_types;
//pub mod foliage_loading_state;
pub mod foliage_assets;
pub mod foliage_proto;
  
pub mod noise;
pub mod foliage_viewer;
 


pub struct BevyFoliageToolPlugin {
    pub foliage_config_path: String 
}  
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

       // .add_plugins(foliage_loading_state::foliage_loading_state_plugin)

       // .add_plugins(foliage::foliage_plugin)

        .add_plugins(foliage_chunk::foliage_chunks_plugin)

        .add_plugins(noise::noise_plugin)

        .add_plugins(foliage_scene::foliage_scene_plugin)
        .add_plugins(foliage_layer::foliage_layer_plugin)


        .add_plugins(bevy_foliage_edits_plugin)
        ; 
        
    }
}



/// This plugin is responsible for attaching models and materials to FoliageProto entities 
pub struct BevyFoliageProtoPlugin  ;
impl Plugin for BevyFoliageProtoPlugin {
    fn build(&self, app: &mut App) {
        app 

      //  .insert_resource( self.foliage_assets_resource.clone() )
        .add_plugins(foliage_assets::foliage_assets_plugin)

        .add_plugins(foliage_proto::foliage_proto_plugin)

        ;
    }
}