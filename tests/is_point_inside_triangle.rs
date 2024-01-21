extern crate sx3d;
use sx3d::utils::{calculate_triangle_area, is_point_inside_triangle};

#[test]
fn it_should_return_true_when_point_is_inside_triangle() {
    let point = [-0.5, -0.5];
    let triangle: [[f32; 2]; 3] = [[0.0, 0.0], [-1.0, 0.0], [0.0, -1.0]];
    let triangle_area = calculate_triangle_area(&triangle);
    let result = is_point_inside_triangle(&point, &triangle, &triangle_area);

    assert!(result);
}

#[test]
fn it_should_return_false_when_point_is_outside_triangle() {
    let point = [0.5, 0.5];
    let triangle: [[f32; 2]; 3] = [[0.0, 0.0], [-1.0, 0.0], [0.0, -1.0]];
    let triangle_area = calculate_triangle_area(&triangle);
    let result = is_point_inside_triangle(&point, &triangle, &triangle_area);

    assert!(!result);
}

#[test]
fn it_should_return_true_when_point_is_on_triangle_edge() {
    let point = [0.0, 0.0];
    let triangle: [[f32; 2]; 3] = [[0.0, 0.0], [-1.0, 0.0], [0.0, -1.0]];
    let triangle_area = calculate_triangle_area(&triangle);
    let result = is_point_inside_triangle(&point, &triangle, &triangle_area);

    assert!(result);
}

#[test]
fn it_should_return_false_when_point_is_outside_near_triangle_edge() {
    let point = [0.0001, 0.0001];
    let triangle: [[f32; 2]; 3] = [[0.0, 0.0], [-1.0, 0.0], [0.0, -1.0]];
    let triangle_area = calculate_triangle_area(&triangle);
    let result = is_point_inside_triangle(&point, &triangle, &triangle_area);

    assert!(!result);
}
