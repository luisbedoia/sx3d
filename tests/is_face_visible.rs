extern crate sx3d;
use sx3d::utils::is_face_visible;

#[test]
fn it_should_return_true_when_face_is_visible() {
    let observer = [0.0, 0.0, -1.0];
    let face_normal = [0.0, 0.0, 1.0];
    let result = is_face_visible(&face_normal, &observer);

    assert!(result);
}

#[test]
fn it_should_return_false_when_face_is_not_visible() {
    let observer = [0.0, 0.0, 1.0];
    let face_normal = [0.0, 0.0, 1.0];
    let result = is_face_visible(&face_normal, &observer);

    assert!(!result);
}

#[test]
fn it_should_return_false_when_face_is_parallel_to_observer() {
    let observer = [-1.0, 0.0, 0.0];
    let face_normal = [0.0, 0.0, 1.0];
    let result = is_face_visible(&face_normal, &observer);

    assert!(!result);
}
