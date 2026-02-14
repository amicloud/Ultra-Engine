use engine::{
    CollisionLayer, ConvexCollider, TransformComponent,
    epa::{EpaResult, epa},
    gjk::{GjkResult, gjk_intersect},
};
use glam::{Mat4, Quat, Vec3};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn run_epa(
    a: &ConvexCollider,
    a_transform: Mat4,
    b: &ConvexCollider,
    b_transform: Mat4,
) -> EpaResult {
    let result = gjk_intersect(a, a_transform, b, b_transform);
    let simplex = match result {
        GjkResult::Intersection(hit) => hit.simplex,
        GjkResult::NoIntersection => panic!("Expected intersection."),
    };

    epa(a, a_transform, b, b_transform, &simplex).expect("EPA failed")
}
fn transform_at(position: Vec3, rotation: Quat) -> Mat4 {
    TransformComponent {
        position,
        rotation,
        scale: Vec3::ONE,
    }
    .to_mat4()
}
#[test]
fn test() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let a = ConvexCollider::cube(2.0, CollisionLayer::Default);
    let b = ConvexCollider::cube(2.0, CollisionLayer::Default);
    let a_transform = transform_at(Vec3::ZERO, Quat::IDENTITY);
    let b_transform = transform_at(Vec3::new(1.0, 0.0, 0.0), Quat::IDENTITY);

    let result = run_epa(&a, a_transform, &b, b_transform);
    assert!(result.penetration_depth > 0.0);

    let stats = dhat::HeapStats::get();

    let total_bytes_from_one_operation = stats.total_bytes;
    dhat::assert!(
        total_bytes_from_one_operation > 0,
        "Expected some heap allocations."
    );

    for _ in 0..100 {
        let result = run_epa(&a, a_transform, &b, b_transform);
        assert!(result.penetration_depth > 0.0);
    }

    let total_bytes_from_one_hundred_operations =
        stats.total_bytes - total_bytes_from_one_operation;
    println!(
        "Total bytes allocated after one operation: {}",
        total_bytes_from_one_operation
    );
    println!(
        "Total bytes allocated after one hundred operations: {}",
        total_bytes_from_one_hundred_operations
    );
    println!("{stats:#?}");
    dhat::assert!(
        total_bytes_from_one_hundred_operations == 0,
        "Expected no heap allocations after the first operation, but found {} bytes allocated.",
        total_bytes_from_one_hundred_operations
    );
}
