use crate::foliage_chunk::{FoliageChunk, FoliageChunkNeedsRebuild};
use crate::FoliageConfigResource;
use bevy::prelude::*;

use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub(crate) fn foliage_layer_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            unpack_foliage_layer_data_components,
            propogate_density_updates,
            handle_foliage_layer_rebuild,
        )
            .chain()
            .in_set(FoliageLayerSystemSet),
    );
}

#[derive(SystemSet, Hash, Clone, Debug, Eq, PartialEq)]
pub struct FoliageLayerSystemSet;

/// There is one foliage layer for each foliage index and it is the parent to many chunks
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct FoliageLayer {
    pub dimensions: IVec2,
    pub chunk_rows: usize,
    pub foliage_index: usize, //refer to the config

                              //pub density_map: FoliageDensityMapU8,
                              //pub base_height_map: FoliageBaseHeightMapU8,
}

#[derive(Component)]
pub struct FoliageLayerNeedsRebuild;

/// A component typically on the FoliageLayer entity
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FoliageBaseHeightMapU16(pub Vec<Vec<u16>>);

impl FoliageBaseHeightMapU16 {
    pub fn new(dimensions: IVec2) -> Self {
        let (width, height) = (dimensions.x as usize, dimensions.y as usize);
        let map = vec![vec![0u16; width]; height];
        Self(map)
    }
}

/// A component typically on the FoliageLayer entity
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FoliageDensityMapU8(pub Vec<Vec<u8>>);

impl FoliageDensityMapU8 {
    pub fn new(dimensions: IVec2) -> Self {
        let (width, height) = (dimensions.x as usize, dimensions.y as usize);
        let map = vec![vec![0u8; width]; height];
        Self(map)
    }
}


#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FoliageBaseNormalMapU16(pub Vec<Vec<u16>>);  // 4 for X, 4 for Z

impl FoliageBaseNormalMapU16 {
    pub fn new(dimensions: IVec2) -> Self {
        let (width, height) = (dimensions.x as usize, dimensions.y as usize);
        let map = vec![vec![0u16; width]; height];
        Self(map)
    }
}

/*

    Density data is 1024 x 1024

    base height data SHOULD be 1024x1024


    terrains height data is stored in chunks , where each is 256 x 256

*/

// this is just used for saving
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct FoliageLayerData {
    pub foliage_index: usize, //refer to the config

    pub density_map: FoliageDensityMapU8,
    pub base_height_map: Option<FoliageBaseHeightMapU16>,
    pub base_normal_map: Option<FoliageBaseNormalMapU16> 
}

impl FoliageLayerData {
    pub fn new(foliage_index: usize, boundary_dimensions: IVec2) -> Self {
        Self {
            foliage_index,
            //	dimensions: boundary_dimensions.clone(),
            density_map: FoliageDensityMapU8::new(boundary_dimensions),
            base_height_map: None, //FoliageBaseHeightMapU8::new( boundary_dimensions )
            base_normal_map: None, 
        }
    }
}

fn unpack_foliage_layer_data_components(
    mut commands: Commands,

    foliage_layer_data_query: Query<(Entity, &FoliageLayerData)>,

    foliage_config_resource: Res<FoliageConfigResource>,
    // foliage_types_resource: Res<FoliageTypesResource>
) {
    // this is a beautiful abomination

    for (foliage_layer_entity, foliage_layer_data) in foliage_layer_data_query.iter() {
        let density_map_data = &foliage_layer_data.density_map;
        let base_height_map_data = &foliage_layer_data.base_height_map;
        let base_normal_map_data = &foliage_layer_data.base_normal_map;
        let foliage_index = &foliage_layer_data.foliage_index;

        let foliage_config = &foliage_config_resource.0;
        let chunk_rows = foliage_config.chunk_rows;

        let dimensions = foliage_config.boundary_dimensions;

        let Some(mut foliage_layer_cmd) = commands.get_entity(foliage_layer_entity) else {
            continue;
        };
        info!("unpacking foliage layer data ");

        foliage_layer_cmd
            .remove::<FoliageLayerData>()
            /*.with_children( |child_builder|
               {
                     for (layer_index,layer_data) in layers_data_array {
                                  let layer_entity = child_builder.spawn(
                                        (
                                           SpatialBundle::default() ,
                                           layer_data.clone()
                                       )
                                      ).id();
                                  foliage_layer_entities_map.insert( *layer_index , layer_entity) ;
                      }
               }
            )*/
            .insert(FoliageLayer {
                dimensions: dimensions.clone(),
                chunk_rows: chunk_rows.clone(),
                foliage_index: foliage_index.clone(),
            })
            .insert(Visibility::default())
            .insert(density_map_data.clone())
            //.insert(base_height_map_data.clone())
            .insert(Name::new("foliage_layer"))
            .insert(FoliageLayerNeedsRebuild)
            .despawn_descendants();

        if let Some(base_height_map_data) = base_height_map_data {
            foliage_layer_cmd.insert(base_height_map_data.clone());
        }

         if let Some(base_normal_map_data) = base_normal_map_data {
            foliage_layer_cmd.insert(base_normal_map_data.clone());
        }

        //spawn foliage chunks ?

        for x in 0..chunk_rows {
            for y in 0..chunk_rows {
                let chunk_offset = IVec2::new(x as i32, y as i32);

                let boundary_dimensions = &dimensions;

                let chunk_dimensions = IVec2::new(
                    boundary_dimensions.x / chunk_rows as i32,
                    boundary_dimensions.y / chunk_rows as i32,
                );

                let chunk_translation = Vec3::new(
                    (chunk_offset.x * chunk_dimensions.x) as f32,
                    0.0,
                    (chunk_offset.y * chunk_dimensions.y) as f32,
                );

                let _new_chunk = commands
                    .spawn((
                        Transform::from_translation(chunk_translation),
                        Visibility::default(),
                        FoliageChunk { chunk_offset },
                        Name::new("foliage_chunk"),
                    ))
                    .set_parent(foliage_layer_entity)
                    .id();
            }
        }
    }
}

fn handle_foliage_layer_rebuild(
    mut commands: Commands,
    foliage_layer_query: Query<
        (
            Entity,
            &FoliageLayer,
            &FoliageDensityMapU8,
           // &FoliageBaseHeightMapU16,
          //  &FoliageBaseNormalMapU8,
            &Children,
        ),
        Added<FoliageLayerNeedsRebuild>,
    >,

    foliage_chunk_query: Query<&FoliageChunk>,
) {
    for (foliage_layer_entity, _foliage_layer, _density_comp, /*_base_height_comp, _base_normal_comp, */ children) in
        foliage_layer_query.iter()
    {
        if let Some(mut cmd) = commands.get_entity(foliage_layer_entity) {
            cmd.remove::<FoliageLayerNeedsRebuild>();
        }

        for child in children {
            if !foliage_chunk_query.get(*child).is_ok() {
                continue;
            };
            if let Some(mut cmd) = commands.get_entity(*child) {
                cmd.insert(FoliageChunkNeedsRebuild);
            }
        }
    }
}

fn propogate_density_updates(
    mut commands: Commands,
    foliage_layer_query: Query<
        Entity,
        (
            Without<FoliageLayerNeedsRebuild>,
            Changed<FoliageDensityMapU8>,
        ),
    >,
) {
    for layer_entity in foliage_layer_query.iter() {
        if let Some(mut cmd) = commands.get_entity(layer_entity) {
            cmd.insert(FoliageLayerNeedsRebuild);
        }
    }
}
