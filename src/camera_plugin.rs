use bevy::prelude::*;
use dolly::prelude::*;

struct DollyCameraController {
    camera: CameraRig,
}

impl DollyCameraController {
    fn new() -> Self {
        let camera = CameraRig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_rotation(1.5))
            .with(Arm::new(dolly::glam::Vec3::Z * 8.0));
        DollyCameraController {
            camera: camera.build(),
        }
    }
}

fn camera_controller(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cameras: Query<(&mut DollyCameraController, &mut Transform)>,
) {
    // Can only controll a single camera for now
    let (mut controller, mut transform) =
        if let Some((controller, transform)) = cameras.iter_mut().next() {
            (controller, transform)
        } else {
            return;
        };

    if keyboard_input.just_pressed(KeyCode::X) {
        controller
            .camera
            .driver_mut::<YawPitch>()
            .rotate_yaw_pitch(-90.0, 0.0);
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        controller
            .camera
            .driver_mut::<YawPitch>()
            .rotate_yaw_pitch(90.0, 0.0);
    }

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
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(camera_controller.system());
    }
}
