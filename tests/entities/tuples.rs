use sx3d::entities::vector_3d::Vector3D;

#[test]
fn test_vector3d_norm() {
    let vector = Vector3D::new(3.0, 4.0, 0.0);
    assert_eq!(vector.norm(), 5.0);
}

#[test]
fn test_dot_product_3d() {
    let vector_1 = Vector3D::new(1.0, 2.0, 3.0);
    let vector_2 = Vector3D::new(4.0, 5.0, 6.0);
    assert_eq!(vector_1.dot_product(&vector_2), 32.0);
}

#[test]
fn test_index() {
    let vector = Vector3D::new(1.0, 2.0, 3.0);
    assert_eq!(vector[0], 1.0);
    assert_eq!(vector[1], 2.0);
    assert_eq!(vector[2], 3.0);
}
