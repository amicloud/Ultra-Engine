mod settings;
mod camera_controller;
mod player_controller;
mod input_controller;

use nalgebra::Vector3;
use rand::random_range;
use std::ffi::OsStr;
use ultramayor_engine::{
    ActiveCamera, CameraComponent, Engine, MaterialComponent, MeshComponent, TransformComponent,
    VelocityComponent,
};
use camera_controller::{apply_flying_camera_input, FlyingCameraComponent};
use input_controller::{update_input_state, InputState};
use player_controller::{apply_player_input, PlayerComponent};

use crate::camera_controller::{OrbitCameraComponent, apply_orbit_camera_input};
fn main() {
    println!("Welcome to the Game!");
    let mut engine = Engine::new();

    // Create an ECS-driven camera entity and mark it active.
    let aspect_ratio = 1024.0 / 769.0;
    let camera_transform = TransformComponent{
        position: Vector3::new(0.0, 0.0, 300.0),
        rotation: nalgebra::UnitQuaternion::identity().inverse(),
        scale: Vector3::new(1.0, 1.0, 1.0),
    };

    let flying_camera = engine
        .world
        .spawn((
            camera_transform,
            CameraComponent {
                fov_y_radians: 75.0_f32.to_radians(),
                aspect_ratio,
                near: 0.1,
                far: 1000.0,
            },
            FlyingCameraComponent {
                yaw: -135.0,
                pitch: -45.0,
                sensitivity: 0.1,
            },
            VelocityComponent {
                translational: Vector3::new(0.0, 0.0, 0.0),
                angular: Vector3::new(0.0, 0.0, 0.0),
            },
            PlayerComponent { speed: 100.0 },
        ))
        .id();


    #[allow(unused_variables)]
    let orbit_camera = engine
        .world
        .spawn((
            camera_transform,
            CameraComponent {
                fov_y_radians: 75.0_f32.to_radians(),
                aspect_ratio,
                near: 0.1,
                far: 1000.0,
            },
            OrbitCameraComponent {
                target: Vector3::new(0.0, 0.0, 0.0),
                distance: 100.0,
                yaw: -135.0,
                pitch: -30.0,
                sensitivity: 0.2,
            },
        ))
        .id();

    engine.world.get_resource_mut::<ActiveCamera>().unwrap().set(flying_camera);
    engine.world.insert_resource(InputState::default());
    engine
        .schedule
        .add_systems((update_input_state, apply_orbit_camera_input, apply_flying_camera_input, apply_player_input));

    let assets = [
        // engine.load_gltf(OsStr::new("resources/models/cube/Cube.gltf")),
        // engine.load_gltf(OsStr::new(
        //     "resources/models/normal_tangent_test/NormalTangentMirrorTest.gltf",
        // )),
        // engine.load_gltf(OsStr::new("resources/models/suzanne/Suzanne.gltf")),

        engine.load_gltf(OsStr::new("resources/models/avocado/Avocado.gltf")),
    ];

    let ground = engine.load_gltf(OsStr::new("resources/models/opalton/opalton3Dterrain.gltf"));

    let t_range = 2.0;
    for _ in 0..100 {
        for (mesh_handle, material_handle) in &assets {
            // Random position
            let pos = Vector3::new(
                random_range(-10.0..10.0),
                random_range(-10.0..10.0),
                random_range(-10.0..10.0),
            );

            // Random translational velocity
            let translational = Vector3::new(
                random_range(-t_range..t_range),
                random_range(-t_range..t_range),
                random_range(-t_range..t_range),
            );
            // let translational = Vector3::new(0.0, 0.0, 0.0);

            // Random angular velocity
            let angular = Vector3::new(
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
            );

            let scale = 100.0;
            // Spawn test objects
            engine.world.spawn((
                TransformComponent {
                    position: pos,
                    rotation: nalgebra::UnitQuaternion::identity(),
                    scale: Vector3::new(scale, scale, scale),
                },
                VelocityComponent {
                    translational,
                    angular,
                },
                MeshComponent {
                    mesh_id: *mesh_handle,
                },
                MaterialComponent {
                    material_id: *material_handle,
                },
            ));
        }
    }

    let ground_scale = 1.0;
    engine.world.spawn((
        TransformComponent {
            position: Vector3::new(0.0, 0.0, 10.0),
            rotation: nalgebra::UnitQuaternion::identity(),
            scale: Vector3::new(ground_scale, ground_scale, 1.0),
        },
        MeshComponent {
            mesh_id: ground.0,
        },
        MaterialComponent {
            material_id: ground.1,
        },
    ));

    engine.run();
}
