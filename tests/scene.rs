extern crate sx3d;
use std::f32::consts::PI;

use sx3d::input_output::read_mesh;
use sx3d::scene::{Object, Scene};

#[test]
fn it_should_create_object() {
    let path = "./examples/cube_ascii.stl".to_string();
    let indexed_mesh = read_mesh(path).unwrap();
    let object = Object::new(indexed_mesh.clone());

    let mut scene = Scene::new(object, [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    scene.rotate_delta_y(&(PI / 4.0));
    scene.rotate_delta_z(&(-PI / 4.0));
    let visible_mesh = scene.get_visible_mesh();

    println!("{:#?}", visible_mesh);
}
