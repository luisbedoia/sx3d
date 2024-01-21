extern crate sx3d;
use std::f32::consts::PI;
use sx3d::input_output::read_mesh;
use sx3d::scene::Object;

#[test]
fn it_should_create_object() {
    let path = "./examples/cube_ascii.stl".to_string();
    let indexed_mesh = read_mesh(path).unwrap();
    let object = Object::new(indexed_mesh.clone());
    let expected_maximum_radius = 75.0_f32.sqrt();

    assert_eq!(object.get_maximum_radius(), &expected_maximum_radius);
}

#[test]
fn it_should_rotate_the_object() {
    let path = "./examples/cube_ascii.stl".to_string();
    let indexed_mesh = read_mesh(path).unwrap();
    let mut object = Object::new(indexed_mesh.clone());
    let expected_maximum_radius = 75.0_f32.sqrt();
    let control_vertex_index = 3;

    let angle = PI / 2.0;

    assert_eq!(object.get_maximum_radius(), &expected_maximum_radius);

    let vertex_before = object.get_mesh().vertices[control_vertex_index];
    let x_vertex_before = vertex_before[0];
    let y_vertex_before = vertex_before[1];
    let z_vertex_before = vertex_before[2];

    object.rotate_delta_x(&angle);
    object.rotate_delta_y(&angle);
    object.rotate_delta_z(&angle);

    let vertex_after = object.get_mesh().vertices[control_vertex_index];
    let x_vertex_after = vertex_after[0];
    let y_vertex_after = vertex_after[1];
    let z_vertex_after = vertex_after[2];

    assert_eq!(x_vertex_before, x_vertex_after);
    assert_eq!(y_vertex_before, y_vertex_after);
    assert_eq!(z_vertex_before, -z_vertex_after);

    assert!(object.get_mesh().validate().is_ok());
}
