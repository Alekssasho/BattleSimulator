mod camera_plugin;

use bevy::{prelude::*, diagnostic::LogDiagnosticsPlugin};

fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //commands.spawn_scene(asset_server.load("../../TempoEngine/car_no_tempest.gltf#Scene0"));
    commands.spawn_bundle(camera_plugin::DollyCameraBundle::new());
    let theta = std::f32::consts::FRAC_PI_4;
    let light_transform = Mat4::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::FRAC_PI_4, -theta);
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_matrix(light_transform),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    let ground_material = materials.add(StandardMaterial{
        base_color: Color::GRAY,
        perceptual_roughness: 1.0,
        ..Default::default()
    });

    let box_material = materials.add(StandardMaterial{
        base_color: Color::YELLOW,
        ..Default::default()
    });

    let plane_mesh = meshes.add(Mesh::from(shape::Plane{
        size: 100.0
    }));

    let box_mesh = meshes.add(Mesh::from(shape::Cube{
        size: 1.0
    }));

    // Spawn ground
    commands.spawn_bundle(PbrBundle{
        mesh: plane_mesh.clone(),
        material: ground_material.clone(),
        ..Default::default()
    });

    // Spawn some boxes
    for x in  0..5 {
        for y in 0..5 {
            commands.spawn_bundle(PbrBundle{
                mesh: box_mesh.clone(),
                material: box_material.clone(),
                transform: Transform::from_xyz(x as f32 * 2.0, 0.5, y as f32 * 2.0),
                ..Default::default()
            });
        }
    }
}

fn main() {
    App::new()
        .insert_resource(bevy::pbr::AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default()) // TODO: Do we need this ?
        .add_plugin(camera_plugin::CameraPlugin)
        .add_startup_system(setup.system())
        .run();
}
