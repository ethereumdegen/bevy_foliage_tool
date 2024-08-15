use crate::regions::RegionPlaneMesh;
use bevy::prelude::*;


use super::regions_material::{RegionsMaterialExtension};

#[derive(Resource, Default)]
pub struct ToolPreviewResource {
    pub tool_coordinates: Vec2,
    pub tool_color: Vec3,
    pub tool_radius: f32,
}




pub fn update_tool_uniforms(
    region_mat_ext_query: Query<&Handle<RegionsMaterialExtension>, With<RegionPlaneMesh>>,

    mut terrain_materials: ResMut<Assets<RegionsMaterialExtension>>,

    tool_preview_resource: Res<ToolPreviewResource>,
) {
    for mat_handle in region_mat_ext_query.iter() {
        if let Some(mat) = terrain_materials.get_mut(mat_handle) {
            mat.extension.tool_preview_uniforms.tool_coordinates =
                tool_preview_resource.tool_coordinates;
            mat.extension.tool_preview_uniforms.tool_color = tool_preview_resource.tool_color;
            mat.extension.tool_preview_uniforms.tool_radius = tool_preview_resource.tool_radius;
        }
    }
}
