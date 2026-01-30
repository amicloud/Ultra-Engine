use bevy_ecs::component::Component;
use nalgebra::Vector3;
use std::ops::{Div, Mul};

#[derive(Default, Component, Clone, Copy)]
pub struct VelocityComponent {
    pub translational: Vector3<f32>,
    pub angular: Vector3<f32>,
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
