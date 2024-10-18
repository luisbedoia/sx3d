use assert_float_eq::*;
use sx3d::entities::triangle_3d::Triangle3D;
use sx3d::entities::vector_3d::Vector3D;

mod is_visible {
    use super::*;

    #[test]
    fn triangle3d_is_visible() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(0.0, 0.0, 1.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let observer = Vector3D::new(0.0, 0.0, -1.0);
        assert!(triangle.is_visible(&observer));
    }

    #[test]
    fn triangle3d_is_not_visible() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(0.0, 0.0, 1.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let observer = Vector3D::new(0.0, 0.0, 1.0);
        assert!(!triangle.is_visible(&observer));
    }

    #[test]
    fn triangle3d_is_parallel_to_observer() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(0.0, 0.0, 1.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let observer = Vector3D::new(-1.0, 0.0, 0.0);
        assert!(!triangle.is_visible(&observer));
    }
}

mod shadow_value {
    use super::*;

    #[test]
    fn it_should_calculate_shadow_value_at_180_1() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(1.0, 0.0, 0.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let light = Vector3D::new(-1.0, 0.0, 0.0);

        assert_f32_near!(triangle.shadow_value(&light), 1.0);
    }

    #[test]
    fn it_should_calculate_shadow_value_at_180_2() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(0.0, 1.0, 0.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let light = Vector3D::new(0.0, -1.0, 0.0);

        assert_f32_near!(triangle.shadow_value(&light), 1.0);
    }

    #[test]
    fn it_should_calculate_shadow_value_at_180_3() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(1.0, 1.0, 1.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let light = Vector3D::new(-1.0, -1.0, -1.0);

        assert_f32_near!(triangle.shadow_value(&light), 1.0);
    }

    #[test]
    fn it_should_calculate_shadow_value_at_0_1() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(1.0, 0.0, 0.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let light = Vector3D::new(1.0, 0.0, 0.0);

        assert_f32_near!(triangle.shadow_value(&light), -1.0);
    }

    #[test]
    fn it_should_calculate_shadow_value_at_0_2() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(0.0, 1.0, 0.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let light = Vector3D::new(0.0, 1.0, 0.0);

        assert_f32_near!(triangle.shadow_value(&light), -1.0);
    }

    #[test]
    fn it_should_calculate_shadow_value_at_0_3() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(1.0, 1.0, 1.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let light = Vector3D::new(1.0, 1.0, 1.0);

        assert_f32_near!(triangle.shadow_value(&light), -1.0);
    }

    #[test]
    fn it_should_calculate_shadow_value_at_135_1() {
        let vertex = Vector3D::new(0.0, 0.0, 0.0);
        let triangle = Triangle3D {
            normal: Vector3D::new(-1.0, 0.0, 0.0),
            vertices: [vertex, vertex, vertex],
            vertices_indices: [0, 1, 2],
        };
        let light = Vector3D::new(0.5_f32.sqrt(), 0.0, 0.5_f32.sqrt());

        assert_f32_near!(triangle.shadow_value(&light), 2.0_f32.sqrt() / 2.0);
    }
}

#[test]
fn test_triangle3d_mean_z() {
    let triangle = Triangle3D {
        normal: Vector3D::new(0.0, 0.0, 1.0),
        vertices: [
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(1.0, 0.0, 2.0),
            Vector3D::new(0.0, 1.0, 3.0),
        ],
        vertices_indices: [0, 1, 2],
    };
    assert_eq!(triangle.mean_z(), 2.0);
}
