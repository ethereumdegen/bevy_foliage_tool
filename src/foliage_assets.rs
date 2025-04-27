use crate::foliage_material::FoliageMaterialExtension;
use std::default;

use bevy::prelude::*;
use bevy::platform::collections::hash_map::HashMap;

pub(crate) fn foliage_assets_plugin(app: &mut App) {
    app.init_state::<FoliageAssetsState>()
        .init_resource::<FoliageAssetsResource>();
}

#[derive(Debug, Clone)]
pub enum FoliageMaterialHandle {
    //improve this ?
    Standard(Handle<StandardMaterial>),
    Extended(Handle<FoliageMaterialExtension>), // Add other material types here
                                                // Custom(Handle<CustomMaterial>),
}

#[derive(Resource, Debug, Clone, Default)]
pub struct FoliageAssetsResource {
    pub foliage_mesh_handles: HashMap<String, Handle<Mesh>>,
    //pub foliage_material_handles: HashMap<String, Handle<StandardMaterial>>,
    pub foliage_material_handles: HashMap<String, FoliageMaterialHandle>,

   
}

impl FoliageAssetsResource {
    pub fn register_foliage_mesh(&mut self, name: impl ToString, handle: Handle<Mesh>) {
        self.foliage_mesh_handles.insert(name.to_string(), handle);
    }

    pub fn register_foliage_material(
        &mut self,
        name: impl ToString,
        handle: FoliageMaterialHandle,
    ) {
        self.foliage_material_handles
            .insert(name.to_string(), handle);
    }
}

#[derive(Debug, Clone, Default, States, PartialEq, Eq, Hash)]
pub enum FoliageAssetsState {
    #[default]
    Init,
    Loaded,
}
