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
        (handle_chunk_rebuilds, update_chunk_visibility).chain(), // .in_set(FoliageChunkSystemSet)
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
        &FoliageBaseHeightMapU16,
    )>, //chunks parent should have terrain data

    foliage_types_resource: Res<FoliageTypesResource>,

    foliage_config_resource: Res<FoliageConfigResource>,

    noise_resource: Res<NoiseResource>,
    image_assets: Res<Assets<Image>>,
) {
    for (chunk_entity, foliage_chunk, parent) in chunks_query.iter() {
        let parent_entity = parent.get();

        let Some((foliage_layer, foliage_density_map_comp, foliage_base_height_comp)) =
            foliage_layer_query.get(parent_entity).ok()
        else {
            continue;
        };

        if let Some(mut cmd) = commands.get_entity(chunk_entity) {
            cmd.despawn_descendants()
                .remove::<FoliageChunkNeedsRebuild>();
        }

        let density_map = &foliage_density_map_comp.0;
        let base_height_map = &foliage_base_height_comp.0;

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
                let chunk_base_height_at_point =
                    base_height_map[data_y_index as usize][data_x_index as usize];

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


                let custom_rotation = Quat::from_rotation_y( 
                 noise_sample_scaled * std::f32::consts::PI   //noise based rotation 
                 + (  random_float_y )  // rotation based on the layer to prevent z-fighting 
                   );

                commands
                    .spawn((
                        Transform::from_translation(foliage_proto_translation).with_rotation( custom_rotation ),
                        FoliageProtoBundle::new(foliage_type_definition.clone()),
                        Name::new("foliage_proto"),
                        Visibility::default(),
                    ))
                    .set_parent(chunk_entity);
            }
        }
    }
}
