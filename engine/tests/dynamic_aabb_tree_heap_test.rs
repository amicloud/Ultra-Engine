use bevy_ecs::entity::Entity;
use engine::{Aabb, physics::dynamic_aabb_tree::DynamicAabbTree};
use glam::Vec3;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn make_aabb(center: Vec3, half_extent: f32) -> Aabb {
    Aabb {
        min: center - Vec3::splat(half_extent),
        max: center + Vec3::splat(half_extent),
    }
}

#[test]
fn test() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let mut tree = DynamicAabbTree::default();
    let mut node_ids = Vec::new();

    // Warm up: allocate and build internal structure once.
    for i in 0..64u64 {
        let x = i as f32 * 2.0;
        let id = tree.allocate_leaf(
            Entity::from_bits(10_000 + i),
            make_aabb(Vec3::new(x, 0.0, 0.0), 0.5),
        );
        node_ids.push(id);
    }

    // One update outside the fat AABB to force a remove+reinsert cycle.
    tree.update(node_ids[0], make_aabb(Vec3::new(200.0, 0.0, 0.0), 0.5));

    let warmup_stats = dhat::HeapStats::get();
    let warmup_total_bytes = warmup_stats.total_bytes;
    dhat::assert!(
        warmup_total_bytes > 0,
        "Expected some allocations during warmup."
    );

    // Steady state: updates and queries should not grow heap usage.
    for iter in 0..100 {
        let shift = 200.0 + iter as f32;
        tree.update(node_ids[0], make_aabb(Vec3::new(shift, 0.0, 0.0), 0.5));

        tree.query(make_aabb(Vec3::new(shift, 0.0, 0.0), 5.0), |_entity| {});
    }

    let final_stats = dhat::HeapStats::get();
    let steady_state_allocated = final_stats.total_bytes - warmup_total_bytes;
    println!("Warmup allocated bytes: {}", warmup_total_bytes);
    println!("Steady-state allocated bytes: {}", steady_state_allocated);
    println!("Final heap stats: {final_stats:#?}");

    dhat::assert!(
        steady_state_allocated == 0,
        "Expected zero additional allocations after warmup, but observed {} bytes.",
        steady_state_allocated
    );
}
