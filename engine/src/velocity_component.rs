use bevy_ecs::component::Component;
use glam::Vec3;
use std::ops::{Div, Mul};

#[derive(Component, Default, Clone, Copy)]
pub struct VelocityComponent {
    pub translational: Vec3,
    pub angular: Vec3,
}

impl Mul<f32> for VelocityComponent {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            translational: self.translational * rhs,
            angular: self.angular * rhs,
        }
    }
}

impl Div<f32> for VelocityComponent {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            translational: self.translational / rhs,
            angular: self.angular / rhs,
        }
    }
}
