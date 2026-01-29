use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::*;
use nalgebra::{Matrix3, UnitQuaternion, Vector3};
use ultramayor_engine::{ActiveCamera, InputMessage, MouseButton, TransformComponent};

/// Orbit-style camera parameters controlled by input.
#[derive(Component, Debug)]
#[require(TransformComponent)]
pub struct CameraOrbit {
    pub target: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub sensitivity: f32,
}

impl CameraOrbit {
    /// Updates a transform to match this orbit camera state.
    pub fn apply_to_transform(&self, transform: &mut TransformComponent) {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        let direction = Vector3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            yaw_rad.sin() * pitch_rad.cos(),
            pitch_rad.sin(),
        )
        .normalize();

        transform.position = self.target - (direction * self.distance);

        let forward = (self.target - transform.position).normalize();
        let world_up = Vector3::new(0.0, 0.0, -1.0);
        let right = forward.cross(&world_up).normalize();
        let up = right.cross(&forward).normalize();

        // Camera looks down -Z, so map local -Z to the forward direction explicitly.
        let rotation_matrix = Matrix3::from_columns(&[right, up, -forward]);
        transform.rotation = UnitQuaternion::from_matrix(&rotation_matrix);
    }

    fn right(&self) -> Vector3<f32> {
        let forward = self.direction();
        let world_up = Vector3::new(0.0, 0.0, -1.0);
        forward.cross(&world_up).normalize()
    }

    fn up(&self) -> Vector3<f32> {
        let forward = self.direction();
        self.right().cross(&forward).normalize()
    }

    fn direction(&self) -> Vector3<f32> {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        Vector3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            yaw_rad.sin() * pitch_rad.cos(),
            pitch_rad.sin(),
        )
        .normalize()
    }

    fn pitch_yaw(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * self.sensitivity;
        self.pitch += delta_y * self.sensitivity;
        self.pitch = self.pitch.clamp(-89.9, 89.9);
    }

    fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = self.right();
        let up = self.up();
        let pan_scale = self.distance * (self.sensitivity * self.sensitivity);

        self.target -= (right * delta_x * self.sensitivity) * pan_scale;
        self.target -= (up * delta_y * self.sensitivity) * pan_scale;
    }

    fn zoom(&mut self, delta: f32) {
        self.distance -= delta * self.sensitivity;
        self.distance = self.distance.clamp(10.0, 300.0);
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct MouseButtons {
    left: bool,
    middle: bool,
    right: bool,
    other: bool,
    back: bool,
    forward: bool,
}

impl MouseButtons {
    fn set(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.left = pressed,
            MouseButton::Middle => self.middle = pressed,
            MouseButton::Right => self.right = pressed,
            MouseButton::Other => self.other = pressed,
            MouseButton::Back => self.back = pressed,
            MouseButton::Forward => self.forward = pressed,
        }
    }

    fn any_drag(&self) -> bool {
        self.left || self.middle || self.right || self.other || self.back || self.forward
    }
}

/// Stateful camera input tracking used by the game crate.
#[derive(Copy, Clone, Debug, Default, Resource)]
pub struct CameraInputState {
    last_pos: Option<(f32, f32)>,
    buttons: MouseButtons,
}

/// Applies camera input to the active camera entity.
pub fn apply_camera_input(
    mut reader: MessageReader<InputMessage>,
    active_camera: Res<ActiveCamera>,
    mut input_state: ResMut<CameraInputState>,
    mut query: Query<(&mut TransformComponent, &mut CameraOrbit)>,
) {
    let Some(camera_entity) = active_camera.0 else {
        for _ in reader.read() {}
        return;
    };

    let Ok((mut transform, mut orbit)) = query.get_mut(camera_entity) else {
        for _ in reader.read() {}
        return;
    };

    for message in reader.read() {
        match *message {
            InputMessage::MouseButtonDown { button } => {
                input_state.buttons.set(button, true);
            }
            InputMessage::MouseButtonUp { button } => {
                input_state.buttons.set(button, false);
                if !input_state.buttons.any_drag() {
                    input_state.last_pos = None;
                }
            }
            InputMessage::MouseMove { x, y } => {
                let delta = input_state.last_pos.map(|(lx, ly)| (x - lx, y - ly));
                input_state.last_pos = Some((x, y));

                if let Some((dx, dy)) = delta {
                    if input_state.buttons.left {
                        orbit.pitch_yaw(dx, dy);
                    } else if input_state.buttons.middle {
                        orbit.pan(dx, -dy);
                    } else if input_state.buttons.right {
                        orbit.zoom(dy);
                    }
                }
            }
            InputMessage::MouseScroll { delta } => {
                orbit.zoom(delta);
            }
            InputMessage::KeyDown { .. } | InputMessage::KeyUp { .. } => {}
        }
    }

    orbit.apply_to_transform(&mut transform);
}
