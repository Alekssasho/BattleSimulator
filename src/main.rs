mod camera;
mod animation;

use std::time::Duration;

use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_mod_picking::*;

fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //commands.spawn_scene(asset_server.load("../../TempoEngine/car_no_tempest.gltf#Scene0"));
    commands
        .spawn_bundle(camera::DollyCameraBundle::new())
        .insert_bundle(PickingCameraBundle::default());
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

    let ground_material = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        perceptual_roughness: 1.0,
        ..Default::default()
    });

    let box_material = materials.add(StandardMaterial {
        base_color: Color::YELLOW,
        ..Default::default()
    });

    let plane_mesh = meshes.add(Mesh::from(shape::Plane { size: 100.0 }));

    let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Spawn ground
    commands
        .spawn_bundle(PbrBundle {
            mesh: plane_mesh.clone(),
            material: ground_material.clone(),
            ..Default::default()
        })
        .insert(PickableMesh::default());

    // Spawn some boxes
    for x in 0..5 {
        for y in 0..5 {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: box_mesh.clone(),
                    material: box_material.clone(),
                    transform: Transform::from_xyz(x as f32 * 2.0, 0.5, y as f32 * 2.0),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                // TODO: Maybe add animation components at runtime, and remove the afterwards
                .insert(animation::MoveTo{
                    target: Vec3::ZERO,
                    time: Duration::ZERO,
                });
        }
    }
}

fn move_selection_to_position(
    mouse_state: Res<Input<MouseButton>>,
    camera_query: Query<&PickingCamera>,
    mut objects: Query<(&mut animation::MoveTo, &Selection)>

) {
    if !mouse_state.just_pressed(MouseButton::Right) {
        return;
    }

    let camera = camera_query.get_single().unwrap();
    // TODO: Search for ground mesh only
    if let Some((_, i)) = camera.intersect_top() {
        let position_to_go = i.position();

        for (mut move_to, selection) in objects.iter_mut() {
            if selection.selected() {
                move_to.target = position_to_go;
                move_to.time = Duration::from_secs(2);
            }
        }
    }
}


fn main() {
    App::new()
        .insert_resource(bevy::pbr::AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default()) // TODO: Do we need this ?
        // Our own plugins
        .add_plugin(camera::CameraPlugin)
        .add_plugin(animation::AnimationPlugin)
        // Third Party Bevy plugins
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        // Startup Systems
        .add_startup_system(setup)
        // Normal Systems
        .add_system(move_selection_to_position)
        .run();
}
