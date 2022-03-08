use std::{time::Duration};

use bevy::prelude::*;

#[derive(Component)]
pub struct MoveTo {
    pub target: Vec3,
    pub time: Duration,
}

fn animate_move_to(
    time: Res<Time>,
    mut objects: Query<(&mut GlobalTransform, &mut MoveTo)>,
) {
    for (mut transform, mut target) in objects.iter_mut() {
        if target.time.is_zero() {
            continue;
        }

        let current_delta = time.delta_seconds();
        let alpha = (current_delta / target.time.as_secs_f32()).clamp(0.0, 1.0);
        let current_pos = transform.translation;
        let target_pos = target.target;

        let new_position = current_pos.lerp(target_pos, alpha);
        transform.translation = new_position;

        if target.time < time.delta() {
            target.time = Duration::ZERO;
        } else {
            target.time -= time.delta();
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_move_to);
    }
}