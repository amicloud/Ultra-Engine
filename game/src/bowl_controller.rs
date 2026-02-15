// use bevy_ecs::prelude::*;
// use engine::VelocityComponent;

// #[derive(Component, Debug, Clone, Copy)]
// pub struct BowlFloatComponent {
//     pub base_height: f32,
//     pub amplitude: f32,
//     pub speed: f32,
// }

// #[derive(Resource, Debug, Default)]
// pub struct BowlFloatTime {
//     pub seconds: f32,
// }

// pub fn update_bowl_float(
//     mut time: ResMut<BowlFloatTime>,
//     mut query: Query<(&BowlFloatComponent, &mut VelocityComponent)>,
// ) {
//     const FIXED_DT: f32 = 1.0 / 60.0;
//     time.seconds += FIXED_DT;

//     for (bowl, mut velocity) in &mut query {
//         let offset = triangle_wave(time.seconds * bowl.speed, 1.0) * bowl.amplitude;
//         velocity.translational.z = bowl.base_height + offset;
//         // velocity.angular.y = sawtooth_wave(time.seconds * bowl.speed, 5.0) * bowl.amplitude/100.0;
//     }
// }

// fn triangle_wave(time: f32, period: f32) -> f32 {
//     let t = time % period;
//     if t < period / 2.0 {
//         (t / (period / 2.0)) * 2.0 - 1.0
//     } else {
//         ((period - t) / (period / 2.0)) * 2.0 - 1.0
//     }
// }
