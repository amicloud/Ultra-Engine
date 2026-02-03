use bevy_ecs::prelude::{Changed, Entity, Query, Res, ResMut, With};
use glam::Vec3;
use std::collections::HashMap;

use crate::{
    TransformComponent,
    collider_component::ColliderComponent,
    mesh::AABB,
    physics_component::PhysicsType,
    physics_component::PhysicsComponent,
    physics_resource::{Contact, Impulse, PhysicsResource},
    velocity_component::VelocityComponent,
};

#[derive(Default)]
pub struct CollisionSystem {}

impl CollisionSystem {
    pub fn update_world_aabb_cache(
        query: Query<
            (Entity, &ColliderComponent, &TransformComponent),
            Changed<TransformComponent>,
        >,
        mut phys: ResMut<PhysicsResource>,
    ) {
        for (entity, collider, transform) in &query {
            let world_aabb = transform_aabb(collider.aabb(), transform);
            phys.world_aabbs.insert(entity, world_aabb);
        }
    }

    pub fn generate_contacts(
        moving_query: Query<
            (Entity, &ColliderComponent, &TransformComponent),
            Changed<TransformComponent>,
        >,
        all_query: Query<(Entity, &ColliderComponent), With<ColliderComponent>>,
        mut phys: ResMut<PhysicsResource>,
    ) {
        // Collect moving entities
        let moving_entities: Vec<Entity> = moving_query.iter().map(|(e, _, _)| e).collect();
        let mut contacts = Vec::new();

        // Iterate over moving entities only
        for &entity_a in &moving_entities {
            let Some(aabb_a) = phys.world_aabbs.get(&entity_a) else {
                continue;
            };

            // Compare against all colliders (moving + static)
            for (entity_b, _collider_b) in &all_query {
                if entity_a == entity_b {
                    continue;
                } // skip self

                let Some(aabb_b) = phys.world_aabbs.get(&entity_b) else {
                    continue;
                };

                if aabb_intersects(aabb_a, aabb_b) {
                    // Simple approximation: normal along the largest penetration axis
                    let delta = (aabb_b.min + aabb_b.max) * 0.5 - (aabb_a.min + aabb_a.max) * 0.5;
                    let overlap_x = (aabb_a.max.x - aabb_a.min.x + aabb_b.max.x - aabb_b.min.x)
                        * 0.5
                        - delta.x.abs();
                    let overlap_y = (aabb_a.max.y - aabb_a.min.y + aabb_b.max.y - aabb_b.min.y)
                        * 0.5
                        - delta.y.abs();
                    let overlap_z = (aabb_a.max.z - aabb_a.min.z + aabb_b.max.z - aabb_b.min.z)
                        * 0.5
                        - delta.z.abs();

                    let (penetration, normal) = if overlap_x < overlap_y && overlap_x < overlap_z {
                        (overlap_x, Vec3::new(delta.x.signum(), 0.0, 0.0))
                    } else if overlap_y < overlap_z {
                        (overlap_y, Vec3::new(0.0, delta.y.signum(), 0.0))
                    } else {
                        (overlap_z, Vec3::new(0.0, 0.0, delta.z.signum()))
                    };

                    contacts.push(Contact {
                        entity_a,
                        entity_b,
                        normal,
                        penetration,
                    });
                }
            }
        }

        for contact in contacts {
            phys.add_contact(contact);
        }
    }

    pub fn resolve_contacts(
        query: Query<(Option<&VelocityComponent>, Option<&PhysicsComponent>)>,
        mut phys: ResMut<PhysicsResource>,
        mut transforms: Query<&mut TransformComponent>,
    ) {
        let mut impulses = Vec::new();
        let mut corrections: HashMap<Entity, Vec3> = HashMap::new();
        for contact in phys.contacts.iter() {
            let (vel_a, phys_a) = query
                .get(contact.entity_a)
                .map(|(v, p)| (v.map(|v| v.translational), p))
                .unwrap_or((None, None));
            let (vel_b, phys_b) = query
                .get(contact.entity_b)
                .map(|(v, p)| (v.map(|v| v.translational), p))
                .unwrap_or((None, None));

            let velocity_a = vel_a.unwrap_or(Vec3::ZERO);
            let velocity_b = vel_b.unwrap_or(Vec3::ZERO);

            let inv_mass_a = phys_a
                .filter(|p| matches!(p.physics_type, PhysicsType::Dynamic))
                .map(|p| if p.mass > 0.0 { 1.0 / p.mass } else { 0.0 })
                .unwrap_or(0.0);
            let inv_mass_b = phys_b
                .filter(|p| matches!(p.physics_type, PhysicsType::Dynamic))
                .map(|p| if p.mass > 0.0 { 1.0 / p.mass } else { 0.0 })
                .unwrap_or(0.0);
            let inv_mass_sum = inv_mass_a + inv_mass_b;

            if inv_mass_sum == 0.0 {
                continue;
            }

            let restitution = match (phys_a, phys_b) {
                (Some(a), Some(b)) => a.restitution.min(b.restitution),
                (Some(a), None) => a.restitution,
                (None, Some(b)) => b.restitution,
                (None, None) => 0.0,
            };

            let relative_velocity = velocity_b - velocity_a;
            let vel_along_normal = relative_velocity.dot(contact.normal);

            let penetration_slop = 0.01;
            let resting_threshold = 0.2;
            if vel_along_normal.abs() < resting_threshold
                && contact.penetration <= penetration_slop
            {
                continue;
            }
            let effective_restitution = if vel_along_normal.abs() < resting_threshold {
                0.0
            } else {
                restitution
            };

            let mut normal_impulse = 0.0;
            if vel_along_normal < -resting_threshold {
                normal_impulse = (-(1.0 + effective_restitution) * vel_along_normal) / inv_mass_sum;
                let impulse = contact.normal * normal_impulse;
                impulses.push(Impulse {
                    entity: contact.entity_a,
                    linear: -impulse,
                    angular: Vec3::ZERO,
                });
                impulses.push(Impulse {
                    entity: contact.entity_b,
                    linear: impulse,
                    angular: Vec3::ZERO,
                });
            }

            if normal_impulse > 0.0 {
                let tangent = relative_velocity - vel_along_normal * contact.normal;
                let tangent_len = tangent.length();
                if tangent_len > 1e-6 {
                    let tangent_dir = tangent / tangent_len;
                    let jt = -relative_velocity.dot(tangent_dir) / inv_mass_sum;

                    let friction = match (phys_a, phys_b) {
                        (Some(a), Some(b)) => (a.friction * b.friction).sqrt(),
                        (Some(a), None) => a.friction,
                        (None, Some(b)) => b.friction,
                        (None, None) => 0.0,
                    };

                    let max_friction = friction * normal_impulse;
                    let friction_impulse = jt.clamp(-max_friction, max_friction);
                    let impulse = tangent_dir * friction_impulse;
                    impulses.push(Impulse {
                        entity: contact.entity_a,
                        linear: -impulse,
                        angular: Vec3::ZERO,
                    });
                    impulses.push(Impulse {
                        entity: contact.entity_b,
                        linear: impulse,
                        angular: Vec3::ZERO,
                    });
                }
            }

            let correction_percent = 0.2;
            let penetration = (contact.penetration - penetration_slop).max(0.0);
            if penetration > 0.0 {
                let correction =
                    contact.normal * (penetration * correction_percent / inv_mass_sum);

                if inv_mass_a > 0.0 {
                    let entry = corrections.entry(contact.entity_a).or_insert(Vec3::ZERO);
                    *entry -= correction * inv_mass_a;
                }
                if inv_mass_b > 0.0 {
                    let entry = corrections.entry(contact.entity_b).or_insert(Vec3::ZERO);
                    *entry += correction * inv_mass_b;
                }
            }
        }
        for impulse in impulses {
            phys.add_impulse(impulse.entity, impulse.linear, impulse.angular);
        }
        for (entity, correction) in corrections {
            if let Ok(mut transform) = transforms.get_mut(entity) {
                transform.position += correction;
            }
        }
        phys.contacts.clear();
    }
}

fn transform_aabb(local: AABB, transform: &TransformComponent) -> AABB {
    let matrix = transform.to_mat4();
    let min = local.min;
    let max = local.max;

    let corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(min.x, max.y, max.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(max.x, max.y, max.z),
    ];

    let mut world_min = matrix.transform_point3(corners[0]);
    let mut world_max = world_min;

    for corner in corners.iter().skip(1) {
        let world = matrix.transform_point3(*corner);
        world_min = world_min.min(world);
        world_max = world_max.max(world);
    }

    AABB {
        min: world_min,
        max: world_max,
    }
}

fn aabb_intersects(a: &AABB, b: &AABB) -> bool {
    (a.min.x <= b.max.x && a.max.x >= b.min.x)
        && (a.min.y <= b.max.y && a.max.y >= b.min.y)
        && (a.min.z <= b.max.z && a.max.z >= b.min.z)
}
