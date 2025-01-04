use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::prelude::*;

use bevy::asset::Handle;
pub fn noise_plugin(app: &mut App) {
    let asset_server = app.world().resource::<AssetServer>();

    let noise_texture_bytes = include_bytes!("internal_assets/noise1.png");

    let extension = ImageType::Extension("png");
    let compression = CompressedImageFormats::default();
    let is_srgb = false;
    let sampler = ImageSampler::default();
    let noise_texture = Image::from_buffer(
        noise_texture_bytes,
        extension,
        compression,
        is_srgb,
        sampler,
        RenderAssetUsages::default(),
    )
    .unwrap();

    app.insert_resource(NoiseResource {
        density_noise_texture: asset_server.add(noise_texture),
    });
}

#[derive(Resource)]
pub struct NoiseResource {
    pub density_noise_texture: Handle<Image>,
}
