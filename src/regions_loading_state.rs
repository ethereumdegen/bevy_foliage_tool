use bevy::prelude::*;

use crate::chunk::TerrainChunkMesh;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum RegionsLoadingState {
    #[default]
    Initialized,

    Loading,

    Complete,
}
 