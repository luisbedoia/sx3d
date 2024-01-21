extern crate sx3d;
use assert_float_eq::*;
use sx3d::utils::calculate_shadow_value;

#[test]
fn it_should_calculate_shadow_value_at_180_1() {
    let normal = [1.0, 0.0, 0.0];
    let light = [-1.0, 0.0, 0.0];
    let result = calculate_shadow_value(&light, &normal);

    assert_f32_near!(result, 1.0);
}

#[test]
fn it_should_calculate_shadow_value_at_180_2() {
    let normal = [0.0, 1.0, 0.0];
    let light = [0.0, -1.0, 0.0];
    let result = calculate_shadow_value(&light, &normal);

    assert_f32_near!(result, 1.0);
}

#[test]
fn it_should_calculate_shadow_value_at_180_3() {
    let normal = [1.0, 1.0, 1.0];
    let light = [-1.0, -1.0, -1.0];
    let result = calculate_shadow_value(&light, &normal);

    assert_f32_near!(result, 1.0);
}

#[test]
fn it_should_calculate_shadow_value_at_0_1() {
    let normal = [1.0, 0.0, 0.0];
    let light = [1.0, 0.0, 0.0];
    let result = calculate_shadow_value(&light, &normal);

    assert_f32_near!(result, -1.0);
}

#[test]
fn it_should_calculate_shadow_value_at_0_2() {
    let normal = [0.0, 1.0, 0.0];
    let light = [0.0, 1.0, 0.0];
    let result = calculate_shadow_value(&light, &normal);

    assert_f32_near!(result, -1.0);
}

#[test]
fn it_should_calculate_shadow_value_at_0_3() {
    let normal = [1.0, 1.0, 1.0];
    let light = [1.0, 1.0, 1.0];
    let result = calculate_shadow_value(&light, &normal);

    assert_f32_near!(result, -1.0);
}

#[test]
fn it_should_calculate_shadow_value_at_135_1() {
    let normal = [-1.0, 0.0, 0.0];
    let light = [0.5_f32.sqrt(), 0.0, 0.5_f32.sqrt()];
    let result = calculate_shadow_value(&light, &normal);

    assert_f32_near!(result, 2.0_f32.sqrt() / 2.0);
}
