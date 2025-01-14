use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::input::mouse::MouseMotion;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::{pbr::ShadowFilteringMethod, prelude::*};
use bevy_foliage_tool::foliage_assets::FoliageAssetsResource;
use bevy_foliage_tool::foliage_assets::FoliageAssetsState;
use bevy_foliage_tool::foliage_assets::FoliageMaterialHandle;
use bevy_foliage_tool::foliage_layer::FoliageBaseHeightMapU16;
use bevy_foliage_tool::foliage_layer::FoliageLayer;
use bevy_foliage_tool::foliage_layer::FoliageLayerNeedsRebuild;
use bevy_foliage_tool::foliage_material::FoliageMaterialExtension;
use bevy_foliage_tool::foliage_scene::FoliageSceneData;
use bevy_foliage_tool::foliage_viewer::FoliageViewer;
use bevy_foliage_tool::BevyFoliageMaterialPlugin;
use bevy_foliage_tool::BevyFoliageProtoPlugin;
use bevy_foliage_tool::BevyFoliageToolPlugin;

use image::{ImageBuffer, Rgba};

//#[derive(Resource)]
//pub struct TextureLoaderResource {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_obj::ObjPlugin)
        .add_plugins(BevyFoliageToolPlugin {
            foliage_config_path: "assets/foliage/foliage_config.ron".to_string(),
        })
        //only if you want to use the foliage material ext provided
        .add_plugins(BevyFoliageMaterialPlugin)
        //only if you want the plugin to attach mesh and material handles to the protos
        .add_plugins(BevyFoliageProtoPlugin)
        .add_systems(Startup, setup)
        //   .add_systems(Startup,create_and_save_texture)
        .add_systems(Update, update_camera_look)
        .add_systems(Update, update_camera_move)
        .add_systems(Update, update_directional_light_position)
       
        .add_systems(Update, add_height_maps_to_foliage_layers)
        .run();
}


fn add_height_maps_to_foliage_layers(
    mut commands: Commands,
    foliage_layer_query: Query<(Entity, &FoliageLayer), Without<FoliageBaseHeightMapU16>>,
) {
    let terrain_dimensions = 1024;

    for (foliage_layer_entity, foliage_layer) in foliage_layer_query.iter() {
        //  let dimensions = foliage_layer.dimensions.clone();

        let mut combined_heightmap = vec![vec![0u16; terrain_dimensions]; terrain_dimensions];

        /*if combined_height_map.is_empty() {
            warn!("no chunk height data to provide to foliage system");
            continue
        }; */

        let base_height_comp = FoliageBaseHeightMapU16(combined_heightmap);

        commands
            .entity(foliage_layer_entity)
            .try_insert(base_height_comp);

        commands
            .entity(foliage_layer_entity)
            .try_insert(FoliageLayerNeedsRebuild);
    }
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let foliage_scene_name = "world_foliage.foliage";

    let foliage_scenes_folder_path = "assets/foliage/foliage_scenes/";

    let foliage_scene_data = FoliageSceneData::create_or_load(foliage_scenes_folder_path, foliage_scene_name);

   //  foliage_scene_data.save_to_disk(foliage_scenes_folder_path );


    commands
        .spawn(Transform::default())
        .insert(
            foliage_scene_data, //this will be unpacked automagically
        )
        .insert(Name::new(foliage_scene_name.clone()));

    // light
    commands.spawn( ( DirectionalLight {
            //shadow_depth_bias: 0.5,
            //shadow_normal_bias: 0.5,
            color: Color::WHITE,

            ..default()
        },
          Transform::from_xyz(4.0, 6.0, 4.0),
     
    ));
    // light

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 822.12,
    });

    // camera
    commands
        .spawn( (
               DepthPrepass,
                Camera3d::default() ,
                    Transform::from_xyz(30.0, 152.5, 30.0)
                .looking_at(Vec3::new(900.0, 0.0, 900.0), Vec3::Y),
                FoliageViewer // important to add this !! 
            ));


  

        //.insert(TerrainViewer::default())
       // .insert(ShadowFilteringMethod::Jimenez14)
        
}

fn update_camera_look(
    mut event_reader: EventReader<MouseMotion>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &Camera3d)>,
) {
    const MOUSE_SENSITIVITY: f32 = 2.0;

    // Accumulate mouse delta
    let mut delta: Vec2 = Vec2::ZERO;
    for event in event_reader.read() {
        delta += event.delta;
    }

    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
        // let rotation = transform.rotation;

        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= delta.x / 180.0 * MOUSE_SENSITIVITY;
        pitch -= delta.y / 180.0 * MOUSE_SENSITIVITY;
        pitch = pitch.clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
}

fn update_camera_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera3d)>,
) {
    const MOVE_SPEED: f32 = 10.0; // You can adjust this value as needed

    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
        // Move the camera forward if W is pressed
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            transform.translation += forward * MOVE_SPEED;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            let forward = transform.forward();
            transform.translation -= forward * MOVE_SPEED;
        }
    }
}

fn update_directional_light_position(
    mut query: Query<&mut Transform, With<DirectionalLight>>,

    time: Res<Time>,
) {
    let current_time = time.elapsed();

    //   let delta_time = time.delta_seconds();

    let SECONDS_IN_A_CYCLE = 20.0;

    let angle = (current_time.as_millis() as f32 / (SECONDS_IN_A_CYCLE * 1000.0))
        * std::f32::consts::PI
        * 2.0; // Convert time to radians

    let radius = 20.0; // Adjust the radius of the sun's orbit
    let x = angle.cos() * radius;
    let y = angle.sin() * radius + 10.0; // Adjust the height of the sun
    let z = 0.0;

    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
