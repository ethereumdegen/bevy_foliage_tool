use crate::noise::NoiseResource;
use crate::FoliageTypesResource;
use crate::foliage_chunk::{FoliageChunk, FoliageChunkNeedsRebuild};
use crate::FoliageConfigResource;
use bevy::prelude::*;

use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub(crate) fn foliage_chunk_layer_plugin(app: &mut App) {
    app 

     .add_systems(
        Update,
        (
            build_chunk_layers,
            //propogate_density_updates,
            //handle_foliage_layer_rebuild,
        )
            .chain()
          //  .in_set(FoliageLayerSystemSet),

    ); 
}




#[derive(Component)] 
pub struct FoliageChunkLayer {

    pub chunk_id: usize,
    pub layer_index: usize, 

 }

fn build_chunk_layers (

        chunk_layer_query: Query  < ( Entity, &FoliageChunkLayer   ) , Added< FoliageChunkLayer > >,



          foliage_types_resource: Res<FoliageTypesResource>,

    foliage_config_resource: Res<FoliageConfigResource>,

  //  foliage_scene_data_resource: Res<FoliageSceneData > ,  //foliage layer data !!! 

    noise_resource: Res<NoiseResource>,
    image_assets: Res<Assets<Image>>,

){







}



 /*









    for (chunk_entity, foliage_chunk, parent) in chunks_query.iter() {
        let parent_entity = parent.get();

        let Some((foliage_layer, foliage_density_map_comp  )) =
            foliage_layer_query.get(parent_entity).ok()
        else {
            continue;
        };

        if let Some(mut cmd) = commands.get_entity(chunk_entity) {
            cmd.despawn_descendants()
                .remove::<FoliageChunkNeedsRebuild>();
        }

        let density_map = &foliage_density_map_comp.0;
     //   let base_height_map =  foliage_base_height_comp.map(|c| c.0 .as_ref() );
    //    let base_normal_map =  foliage_base_normal_comp.map(|c| c.0 .as_ref() );

        let boundary_dimensions = &foliage_layer.dimensions;
        let chunk_rows = &foliage_layer.chunk_rows;

        let chunk_dimensions = IVec2::new(
            boundary_dimensions.x / *chunk_rows as i32,
            boundary_dimensions.y / *chunk_rows as i32,
        );

     //   let chunk_offset = &foliage_chunk.chunk_offset;

        let chunk_data_offset = IVec2::new(
            chunk_offset.x * chunk_dimensions.x,
            chunk_offset.y * chunk_dimensions.y,
        );

        let foliage_index = &foliage_layer.foliage_index;

        let foliage_types_manifest = &foliage_types_resource.0;
        let Some(foliage_type_definition) = foliage_types_manifest
            .foliage_definitions
            .get(*foliage_index)
        else {
            warn!(
                "Cannot build foliage chunk - missing foliage type definition for index {}",
                foliage_index
            );
            continue;
        };

         let mut rng = rand::thread_rng();


        let foliage_config = &foliage_config_resource.0;
        let height_scale = foliage_config.height_scale;

        let max_chunk_density = 256 as f32;
        let max_noise_value = 256 as f32;

        let noise_texture_handle = &noise_resource.density_noise_texture;
        let noise_texture = image_assets
            .get(noise_texture_handle)
            .expect("no noise texture");

        //   info!("rebuild foliage chunk");

        for x in 0..chunk_dimensions.x {
            for y in 0..chunk_dimensions.y {
                let data_x_index = x + chunk_data_offset.x;
                let data_y_index = y + chunk_data_offset.y;

                let chunk_density_at_point =
                    density_map[data_y_index as usize][data_x_index as usize];
                let chunk_base_height_at_point = base_height_map.as_ref().map( |m: &&Vec<Vec<u16>>| m[data_y_index as usize][data_x_index as usize] ).unwrap_or( 0 );
           
                let chunk_base_normal_at_point:u16 = base_normal_map.as_ref().map( |m: &&Vec<Vec<u16>> | m[data_y_index as usize][data_x_index as usize] ).unwrap_or( 0 );

                if chunk_density_at_point <= 0 {
                    continue;
                };

                let chunk_density_scaled = chunk_density_at_point as f32 / max_chunk_density;

                //this is probably wrong
                let noise_tex_data_index = data_y_index * chunk_dimensions.y + data_x_index;

                let noise_sample_at_point = noise_texture.data[noise_tex_data_index as usize];

                let noise_sample_scaled = noise_sample_at_point as f32 / max_noise_value;
                //  info!("noise_sample_at_point {} {}", noise_sample_at_point , noise_sample_scaled);

                if chunk_density_scaled < noise_sample_scaled {
                    continue;
                }



                // Generate a random floating-point number between 0.0 and 1.0
                let random_float_x: f32 = rng.gen::<f32>() - 0.5;
                let random_float_y: f32 = rng.gen::<f32>() - 0.5; // for rotation in radians 
                let random_float_z: f32 = rng.gen::<f32>() - 0.5;



                let foliage_offset = Vec3::new( noise_sample_scaled , 0.0,  1.0 - noise_sample_scaled ) * 2.0 ;
                // info!("offset {}", foliage_offset);
                // info!("chunk_density_at_point {:?}", chunk_density_at_point);

                //combine with noise here ,  then spawn foliage    proto

             /*   let foliage_proto_translation = Vec3::new(
                    x as f32 + foliage_offset.x + random_float_x,
                    chunk_base_height_at_point as f32 * height_scale,
                    y as f32 + foliage_offset.z + random_float_z,
                ); */


                 let foliage_proto_translation = Vec3::new(
                    x as f32  ,
                    chunk_base_height_at_point as f32 * height_scale,
                    y as f32  ,
                );





                // add X and Z rotation from chunk_base_normal_at_point 
           //    let  decoded_normal = decode_normal( chunk_base_normal_at_point );

               //  let normal_x_rotation = Quat::from_rotation_x( decoded_normal.x ) ;
               // let normal_z_rotation = Quat::from_rotation_z( decoded_normal.y ) ;
             //   let normal_rotation = Quat::from_rotation_arc(Vec3::Y, decoded_normal);


                let custom_y_rotation = Quat::from_rotation_y( 
                 noise_sample_scaled * std::f32::consts::PI   //noise based rotation 
                 +   random_float_y    // rotation based on the layer to prevent z-fighting 
                   );


                let final_rotation = normal_rotation * custom_y_rotation;


                commands
                    .spawn((
                        Transform::from_translation(foliage_proto_translation).with_rotation( final_rotation ),
                        FoliageProtoBundle::new(foliage_type_definition.clone()),
                        Name::new("foliage_proto"),
                        Visibility::default(),
                    ))
                    .set_parent(chunk_entity);
            }
        }
    }





 */