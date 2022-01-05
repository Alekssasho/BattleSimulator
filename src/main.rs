mod camera_plugin;

use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("../../TempoEngine/test2.gltf#Scene0"));
    commands.spawn_bundle(camera_plugin::DollyCameraBundle::new());
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(3.0, 5.0, 3.0),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .insert_resource(bevy::pbr::AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(camera_plugin::CameraPlugin)
        .add_startup_system(setup.system())
        .run();
}
