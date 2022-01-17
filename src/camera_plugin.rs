use bevy::{input::mouse::MouseMotion, prelude::*};
use dolly::prelude::*;

#[derive(Component)]
struct DollyCameraController {
    camera: CameraRig,
}

impl DollyCameraController {
    fn new() -> Self {
        let camera = CameraRig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_position_rotation(1.0, 1.0))
            .with(Position::new(dolly::glam::vec3(0.0, 1.0, 1.0) * 8.0));
        DollyCameraController {
            camera: camera.build(),
        }
    }
}

fn camera_controller(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cameras: Query<(&mut DollyCameraController, &mut Transform)>,
) {
    // Can only controll a single camera for now
    let (mut controller, mut transform) =
        if let Some((controller, transform)) = cameras.iter_mut().next() {
            (controller, transform)
        } else {
            return;
        };

    if mouse_button.pressed(MouseButton::Right) {
        for mouse_delta in mouse_motion_events.iter() {
            controller
                .camera
                .driver_mut::<YawPitch>()
                .rotate_yaw_pitch(-0.3 * mouse_delta.delta.x, -0.3 * mouse_delta.delta.y);
        }
    }

    let mut translation_vector = dolly::glam::Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        translation_vector += -dolly::glam::Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::S) {
        translation_vector += dolly::glam::Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::A) {
        translation_vector += -dolly::glam::Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::D) {
        translation_vector += dolly::glam::Vec3::X;
    }

    translation_vector =
        controller.camera.final_transform.rotation * translation_vector.normalize_or_zero();

    let speed = if keyboard_input.pressed(KeyCode::LShift) {
        50.0
    } else {
        10.0
    };

    controller
        .camera
        .driver_mut::<Position>()
        .translate(translation_vector * speed * time.delta_seconds());

    let camera_transform = controller.camera.update(time.delta_seconds());

    *transform = Transform::from_translation(<[f32; 3]>::from(camera_transform.position).into())
        .looking_at(
            <[f32; 3]>::from(camera_transform.position + camera_transform.forward()).into(),
            <[f32; 3]>::from(camera_transform.up()).into(),
        );
}

#[derive(Bundle)]
pub struct DollyCameraBundle {
    #[bundle]
    perspective: PerspectiveCameraBundle,
    controller: DollyCameraController,
}

impl DollyCameraBundle {
    pub fn new() -> Self {
        let controller = DollyCameraController::new();
        let camera_transform = controller.camera.final_transform;

        DollyCameraBundle {
            perspective: PerspectiveCameraBundle {
                transform: Transform::from_translation(
                    <[f32; 3]>::from(camera_transform.position).into(),
                )
                .looking_at(
                    <[f32; 3]>::from(camera_transform.position + camera_transform.forward()).into(),
                    <[f32; 3]>::from(camera_transform.up()).into(),
                ),
                ..Default::default()
            },
            controller,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_controller);
    }
}
