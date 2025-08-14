use sx3d::{mesh::IndexedMesh3D, IndexedTriangle3D, Object, Vector3D};
use assert_float_eq::*;

#[test]
fn it_should_create_object() {
    let vertices = vec![
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(1.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
    ];
    let triangle_1 = IndexedTriangle3D {
        normal: Vector3D::new(0.0, 0.0, -1.0),
        vertices_indices: [0, 1, 2],
    };

    let triangle_2 = IndexedTriangle3D {
        normal: Vector3D::new(0.0, -1.0, 0.0),
        vertices_indices: [0, 1, 3],
    };

    let triangle_3 = IndexedTriangle3D {
        normal: Vector3D::new(-1.0, 0.0, 0.0),
        vertices_indices: [0, 2, 3],
    };

    let x = (1.0_f32 / 3.0_f32).sqrt();
    let triangle_4 = IndexedTriangle3D {
        normal: Vector3D::new(x, x, x),
        vertices_indices: [1, 2, 3],
    };

    let mesh = IndexedMesh3D {
        vertices,
        triangles: vec![triangle_1, triangle_2, triangle_3, triangle_4],
    };

    let object = Object::new(mesh);

    println!("{:#?}", object);

    let expected = (11.0_f32).sqrt() / 4.0_f32; // sqrt(0.75^2 + 0.25^2 + 0.25^2)
    assert_f32_near!(*object.get_maximum_radius(), expected);
}
