extern crate sx3d;
use sx3d::utils::dot_product;

#[test]
fn it_should_return_positive_when_angles_are_acute() {
    let vector_1 = [1.0, 1.0, 0.0];
    let vector_2 = [1.0, 0.0, 0.0];
    let result = dot_product(&vector_1, &vector_2);

    assert!(result > 0.0);
}

#[test]
fn it_should_return_negative_when_angles_are_obtuse() {
    let vector_1 = [1.0, 1.0, 0.0];
    let vector_2 = [-1.0, 0.0, 0.0];
    let result = dot_product(&vector_1, &vector_2);

    assert!(result < 0.0);
}

#[test]
fn it_should_return_zero_when_angles_are_perpendicular() {
    let vector_1 = [1.0, 0.0, 0.0];
    let vector_2 = [0.0, 1.0, 0.0];
    let result = dot_product(&vector_1, &vector_2);

    assert_eq!(result, 0.0);
}
