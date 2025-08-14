use sx3d::canvas::{shade_to_char, GRAYSCALE_LUT};

#[test]
fn shades_are_clamped() {
    let low = shade_to_char(-10.0);
    let high = shade_to_char(10.0);
    assert_eq!(low, GRAYSCALE_LUT[0]);
    assert_eq!(high, GRAYSCALE_LUT[GRAYSCALE_LUT.len() - 1]);
}

#[test]
fn shade_zero_and_one_boundaries() {
    let z = shade_to_char(0.0);
    let o = shade_to_char(1.0);
    assert_eq!(z, GRAYSCALE_LUT[0]);
    assert_eq!(o, GRAYSCALE_LUT[GRAYSCALE_LUT.len() - 1]);
}
