use crate::foliage_layer::FoliageBaseNormalMapU16;
use crate::foliage_config::FoliageConfigResource;
use crate::foliage_layer::FoliageBaseHeightMapU16;
use crate::foliage_layer::FoliageDensityMapU8;
use crate::foliage_layer::FoliageLayer;
use crate::foliage_layer::FoliageLayerSystemSet;

use rand::Rng;

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

        compute_normals_from_height, 
        handle_chunk_rebuilds,
         update_chunk_visibility


         ).chain(), // .in_set(FoliageChunkSystemSet)
                                                                  // .before(FoliageLayerSystemSet),
    );
}

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
pub struct FoliageChunkSystemSet;

#[derive(Component)]
pub struct FoliageChunk {
    pub chunk_offset: IVec2,
}

#[derive(Component)]
pub struct FoliageChunkNeedsRebuild;

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

        let chunk_dimensions = Vec3::new(64.0, 0.0, 64.0);

        let chunk_center_translation = chunk_translation + chunk_dimensions / 2.0;

        let distance = chunk_center_translation.distance(viewer_translation);

        if distance <= max_render_distance {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn handle_chunk_rebuilds(
    mut commands: Commands,

    chunks_query: Query<(Entity, &FoliageChunk, &Parent), With<FoliageChunkNeedsRebuild>>,

    foliage_layer_query: Query<(
        &FoliageLayer,
        &FoliageDensityMapU8,
        Option<&FoliageBaseHeightMapU16>,
        Option<&FoliageBaseNormalMapU16>,
    )>, //chunks parent should have terrain data

    foliage_types_resource: Res<FoliageTypesResource>,

    foliage_config_resource: Res<FoliageConfigResource>,

    noise_resource: Res<NoiseResource>,
    image_assets: Res<Assets<Image>>,
) {
    for (chunk_entity, foliage_chunk, parent) in chunks_query.iter() {
        let parent_entity = parent.get();

        let Some((foliage_layer, foliage_density_map_comp, foliage_base_height_comp, foliage_base_normal_comp )) =
            foliage_layer_query.get(parent_entity).ok()
        else {
            continue;
        };

        if let Some(mut cmd) = commands.get_entity(chunk_entity) {
            cmd.despawn_descendants()
                .remove::<FoliageChunkNeedsRebuild>();
        }

        let density_map = &foliage_density_map_comp.0;
        let base_height_map =  foliage_base_height_comp.map(|c| c.0 .as_ref() );
        let base_normal_map =  foliage_base_normal_comp.map(|c| c.0 .as_ref() );

        let boundary_dimensions = &foliage_layer.dimensions;
        let chunk_rows = &foliage_layer.chunk_rows;

        let chunk_dimensions = IVec2::new(
            boundary_dimensions.x / *chunk_rows as i32,
            boundary_dimensions.y / *chunk_rows as i32,
        );

        let chunk_offset = &foliage_chunk.chunk_offset;

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

                let foliage_proto_translation = Vec3::new(
                    x as f32 + foliage_offset.x + random_float_x,
                    chunk_base_height_at_point as f32 * height_scale,
                    y as f32 + foliage_offset.z + random_float_z,
                );




                // add X and Z rotation from chunk_base_normal_at_point 
                let  decoded_normal = decode_normal( chunk_base_normal_at_point );

               //  let normal_x_rotation = Quat::from_rotation_x( decoded_normal.x ) ;
               // let normal_z_rotation = Quat::from_rotation_z( decoded_normal.y ) ;
                let normal_rotation = Quat::from_rotation_arc(Vec3::Y, decoded_normal);


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
}



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
}
