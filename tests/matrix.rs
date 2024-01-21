extern crate sx3d;
use std::collections::HashMap;
use std::f32::consts::PI;

use sx3d::input_output::read_mesh;
use sx3d::scene::{Object, Scene, VisibleFace, VisibleMesh};
use sx3d::matrix::Matrix;

#[test]
fn it_should_create_object() {
    let path = "./examples/Twisted_torus.stl".to_string();
    let indexed_mesh = read_mesh(path).unwrap();
    let object = Object::new(indexed_mesh.clone());
    let maximum_diameter = 2.0 * object.get_maximum_radius();
    let matrix = Matrix::new(101, maximum_diameter);

    let mut scene = Scene::new(object, [-1.0, -1.0, -1.0], [0.0, 0.0, -1.0]);
    scene.rotate_delta_y(&(-PI / 4.0));
    // scene.rotate_delta_x(PI/4.0);

    let visible_mesh = scene.get_visible_mesh();
    matrix.project(&visible_mesh);

    let data = matrix.get_frame();

    println!("{}", data);
}

#[test]
fn matrix_should_create_frame() {
    let mut vertices: HashMap<usize, [f32; 3]> = HashMap::new();
    vertices.insert(0, [5.0, 5.0, 0.0]);
    vertices.insert(1, [-5.0, 5.0, 0.0]);
    vertices.insert(2, [-5.0, -5.0, 0.0]);
    vertices.insert(3, [5.0, -5.0, 0.0]);
    let matrix = Matrix::new(5, 10.0);

    let mut face_1 = VisibleFace::new([0, 1, 2], [0.0, 0.0, 1.0]);
    face_1.set_shadow_value(1.0);

    let mut face_2 = VisibleFace::new([0, 2, 3], [0.0, 0.0, 1.0]);
    face_2.set_shadow_value(0.2);

    let visible_mesh =
        VisibleMesh::new_from_vertices_and_visible_faces(vertices, vec![face_1, face_2]);

    matrix.project(&visible_mesh);
    let data = matrix.get_frame();

    println!("{}", data);
}

// #[test]
// fn matrix_should_create_frame_1() {
//     let mut matrix = Matrix::new(5, 4.0);
//     matrix.set(-1, -2, '*');
//     matrix.set(-1, -1, '*');
//     matrix.set(-1, 0, '*');
//     matrix.set(-1, 1, '*');
//     matrix.set(-1, 2, '*');

//     matrix.set(0, -2, '*');
//     matrix.set(0, -1, '*');
//     matrix.set(0, 0, '*');
//     matrix.set(0, 1, '*');
//     matrix.set(0, 2, '*');

//     matrix.set(1, -2, '*');
//     matrix.set(1, -1, '*');
//     matrix.set(1, 0, '*');
//     matrix.set(1, 1, '*');
//     matrix.set(1, 2, '*');

//     matrix.set(2, -2, '*');
//     matrix.set(2, -1, '*');
//     matrix.set(2, 0, '*');
//     matrix.set(2, 1, '*');
//     matrix.set(2, 2, '*');
//     let frame = matrix.get_frame();

//     println!("----\n{}---\n", frame);
// }
