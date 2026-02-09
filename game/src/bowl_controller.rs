use bevy_ecs::prelude::*;
use engine::{TransformComponent, VelocityComponent};
use glam::Quat;

#[derive(Component, Debug, Clone, Copy)]
pub struct BowlFloatComponent {
    pub base_height: f32,
    pub amplitude: f32,
    pub speed: f32,
}

#[derive(Resource, Debug, Default)]
pub struct BowlFloatTime {
    pub seconds: f32,
}

pub fn update_bowl_float(
    mut time: ResMut<BowlFloatTime>,
    mut query: Query<(
        &mut TransformComponent,
        &BowlFloatComponent,
        &mut VelocityComponent,
    )>,
) {
    const FIXED_DT: f32 = 1.0 / 60.0;
    time.seconds += FIXED_DT;

    for (mut transform, float, mut velocity) in &mut query {
        let offset = triangle_wave(time.seconds * float.speed, 1.0) * float.amplitude;
        velocity.translational.z = float.base_height + offset;
        transform.rotation = Quat::from_rotation_x(
            (time.seconds * float.speed).cos() * float.amplitude.to_radians(),
        );
    }
}

fn sawtooth_wave(time: f32, period: f32) -> f32 {
    let t = time % period;
    (t / period) * 2.0 - 1.0
}

fn triangle_wave(time: f32, period: f32) -> f32 {
    let t = time % period;
    if t < period / 2.0 {
        (t / (period / 2.0)) * 2.0 - 1.0
    } else {
        ((period - t) / (period / 2.0)) * 2.0 - 1.0
    }
}
