use bevy::{asset::load_internal_asset, prelude::*};

use bevy::asset::VisitAssetDependencies;

use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use bevy::render::render_asset::RenderAssets;

use bevy::pbr::StandardMaterialFlags;
use bevy::pbr::StandardMaterialUniform;

use bevy::pbr::MaterialExtension;

use bevy::pbr::ExtendedMaterial;

pub const FOLIAGE_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1653284672328047579);

pub type FoliageMaterialExtension = ExtendedMaterial<StandardMaterial, FoliageMaterial>;

pub fn foliage_material_plugin(app: &mut App) {
    load_internal_asset!(
        app,
        FOLIAGE_SHADER_HANDLE,
        "shaders/foliage.wgsl",
        Shader::from_wgsl
    );

    app.add_plugins(MaterialPlugin::<FoliageMaterialExtension>::default());
}

#[derive(Asset, AsBindGroup, TypePath, Clone, Debug, Default)]
pub struct FoliageMaterial {
    
}
 

impl MaterialExtension for FoliageMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(FOLIAGE_SHADER_HANDLE)
    }

    fn deferred_fragment_shader() -> ShaderRef {
        ShaderRef::Handle(FOLIAGE_SHADER_HANDLE)
    } 

    

    fn vertex_shader() -> ShaderRef {
        ShaderRef::Handle(FOLIAGE_SHADER_HANDLE)
    }

    fn deferred_vertex_shader() -> ShaderRef {
        ShaderRef::Handle(FOLIAGE_SHADER_HANDLE)
    }

    //important for proper depth testing
    fn prepass_vertex_shader() -> ShaderRef {
        ShaderRef::Handle(FOLIAGE_SHADER_HANDLE)
    } 

    fn prepass_fragment_shader() -> ShaderRef {
        ShaderRef::Handle(FOLIAGE_SHADER_HANDLE)
    } 
}
