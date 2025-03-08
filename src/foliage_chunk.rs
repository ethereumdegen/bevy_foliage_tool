use crate::foliage_chunk_layer::FoliageChunkLayer;
//use crate::foliage_layer::FoliageBaseNormalMapU16;
use crate::foliage_config::FoliageConfigResource;
//use crate::foliage_layer::FoliageBaseHeightMapU16;
//use crate::foliage_layer::FoliageDensityMapU8;
//use crate::foliage_layer::FoliageLayer;
//use crate::foliage_layer::FoliageLayerSystemSet;
use crate::foliage_scene::FoliageSceneData;

use rand::Rng;
use bevy::utils::HashMap ;

use crate::foliage_proto;
use crate::foliage_proto::FoliageProto;
use crate::foliage_proto::FoliageProtoBundle;
use crate::foliage_viewer::FoliageViewer;
use crate::noise::NoiseResource;
use crate::FoliageTypesResource;
use bevy::prelude::*;

pub(crate) fn foliage_chunks_plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (

     //   compute_normals_from_height, 
       handle_chunk_changed, 
        handle_chunk_rebuilds,
         update_chunk_visibility


         ).chain(), // .in_set(FoliageChunkSystemSet)
                                                                  // .before(FoliageLayerSystemSet),
    );
}

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
pub struct FoliageChunkSystemSet;







#[derive(Component)]
#[require( FoliageChunkLayerChildren )]
pub struct FoliageChunk {
    pub chunk_id: usize ,
}


#[derive(Component)]
pub struct FoliageHeightMapData ( pub Vec<Vec<u16>> );

#[derive(Component)]
pub struct FoliageDimensionsData ( pub IVec2 );



#[derive(Component)]
pub struct FoliageDataSource ( pub Entity );



#[derive(Component,Default)]
pub struct FoliageChunkLayerChildren  ( pub HashMap< usize, Entity  > );











#[derive(Component)]
pub struct FoliageChunkNeedsRebuild;


/*

terrain chunks are 128 x 128 ! 

*/

fn update_chunk_visibility(
    foliage_viewer_query: Query<Entity, With<FoliageViewer>>,

    global_xform_query: Query<&GlobalTransform>,

    mut foliage_chunk_query: Query<(Entity, &FoliageChunk, &mut Visibility)>,

    foliage_config_resource: Res<FoliageConfigResource>,
) {
    let Some(foliage_viewer_entity) = foliage_viewer_query.get_single().ok() else {
        return;
    };

    let Some(viewer_xform) = global_xform_query.get(foliage_viewer_entity).ok() else {
        return;
    };

    let viewer_translation = viewer_xform.translation();

    let foliage_config = &foliage_config_resource.0;

    let Some(max_render_distance) = foliage_config.render_distance else {
        return;
    };

    for (foliage_chunk_entity, _foliage_chunk, mut visibility) in foliage_chunk_query.iter_mut() {
        let Some(chunk_xform) = global_xform_query.get(foliage_chunk_entity).ok() else {
            continue;
        };
        let chunk_translation = chunk_xform.translation();

        let chunk_dimensions = Vec3::new(128.0, 0.0, 128.0);

        let chunk_center_translation = chunk_translation + chunk_dimensions / 2.0;

        let distance = chunk_center_translation.distance(viewer_translation);

        if distance <= max_render_distance {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}


#[derive(Component)]
pub struct ForceRebuildFoliageChunk ;
 



fn handle_chunk_changed(
    mut commands: Commands,
  
    chunk_query: Query<(

        Entity,  

        &FoliageChunk  ,

       // &FoliageLayer,
      //  &FoliageDensityMapU8,  // this is in the foliage scene ! 

        &FoliageHeightMapData,
        &FoliageDimensionsData, 


      //  Option<&FoliageBaseHeightMapU16>,
      //  Option<&FoliageBaseNormalMapU16>,
    ) , Or<(  Changed<FoliageDimensionsData > , Changed<FoliageHeightMapData> ) >>, //chunks parent should have terrain data

  
) {



    /*
        
        insert ForceChunkRebuild !! which deletes all children (chunk layers) and re-creates them ! 

    */

    for (chunk_entity,  foliage_chunk, heightmap, dimensions ) in chunk_query.iter(){

         if let Some(mut cmd) = commands.get_entity( chunk_entity ){


            cmd.insert(  ForceRebuildFoliageChunk );
         }


    }
   






}

fn handle_chunk_rebuilds(

    mut commands: Commands,
  
    chunk_query: Query<(

        Entity,  

        &FoliageChunk  ,

      
        &FoliageHeightMapData,
        &FoliageDimensionsData, 

 
    ) ,    With<ForceRebuildFoliageChunk >  > , //chunks parent should have terrain data



   foliage_scene_query:  Query< &FoliageSceneData >,


    foliage_types_resource: Res<FoliageTypesResource>,

    foliage_config_resource: Res<FoliageConfigResource>,

  //  foliage_scene_data_resource: Res<FoliageSceneData > ,  //foliage layer data !!! 

    noise_resource: Res<NoiseResource>,
     

) {


    // delete all chunk_layer children 

  
      for (chunk_entity,  foliage_chunk, heightmap, dimensions ) in chunk_query.iter(){

          if let Some( mut cmd ) = commands.get_entity( chunk_entity ){
 
                cmd.despawn_descendants();








          }


      }


    let Some(foliage_scene_data) = foliage_scene_query.get_single().ok() else {

        warn!("foliage scene data is not a singleton!? ");

        return 
    };

      //recreate them !! 


    for (layer_index,layer_data) in   foliage_scene_data.foliage_layers.iter() {



              for (chunk_entity,  foliage_chunk, heightmap, dimensions ) in chunk_query.iter(){

                  if let Some( mut cmd ) = commands.get_entity( chunk_entity ){
         
                         



                        commands.spawn(

                            (
                                FoliageChunkLayer {
                                    chunk_id: foliage_chunk.chunk_id,
                                    layer_index: *layer_index 

                                }  


                                //insert density map !? 

                            )


                         ).set_parent(chunk_entity);



                        
                  }


              }





    }









}

/*
fn decode_normal(encoded_normal: u16) -> Vec3 {

     if encoded_normal == 0 {
        // Special case for a flat surface / missing data 
        return Vec3::Y;
    }


    let x = ((encoded_normal >> 8) & 0xFF) as f32 / 255.0 * 2.0 - 1.0; // Extract high 8 bits for x and scale to [-1, 1]
    let z = (encoded_normal & 0xFF) as f32 / 255.0 * 2.0 - 1.0;         // Extract low 8 bits for z and scale to [-1, 1]
    
    Vec3::new(x, 1.0, z).normalize()
}



// put this in a thread pattern ?  kinda cpu intensive
fn compute_normals_from_height(

    mut commands: Commands, 

      foliage_layer_query: Query<(
       // &mut FoliageBaseNormalMapU16,
       Entity,  &FoliageBaseHeightMapU16,
    ), (Changed<FoliageBaseHeightMapU16> , Without<FoliageBaseNormalMapU16> ) >,


) {
    for ( chunk_entity,  height_map) in foliage_layer_query.iter () {
        let height_data = &height_map.0;
        let height = height_data.len();
        let width = height_data[0].len();

        let mut normals = vec![vec![0u16; width]; height];

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let height_left = height_data[y][x - 1] as f32;
                let height_right = height_data[y][x + 1] as f32;
                let height_up = height_data[y - 1][x] as f32;
                let height_down = height_data[y + 1][x] as f32;

                // Compute slope in X and Z directions
                let slope_x = (height_right - height_left) / 2.0;
                let slope_z = (height_down - height_up) / 2.0;

                // Calculate the normal
                let normal = Vec3::new(-slope_x, 1.0, -slope_z).normalize();

                // Encode the normal into a u16 format
                let encoded_normal = encode_normal(normal);

                normals[y][x] = encoded_normal;
            }
        }

     

        if let Some(mut cmd) = commands.get_entity(chunk_entity){
              info!("built  normal map ");
            cmd.insert((

                FoliageBaseNormalMapU16 ( normals ),

                FoliageChunkNeedsRebuild 

                ));
        }
    }
}

fn encode_normal(normal: Vec3) -> u16 {
    // Convert a normal vector to an encoded u16 representation
    let x = ((normal.x.clamp(-1.0, 1.0) + 1.0) / 2.0 * 255.0).round() as u16;
    let z = ((normal.z.clamp(-1.0, 1.0) + 1.0) / 2.0 * 255.0).round() as u16;

    (x << 8) | z
}*/
