pub fn get_shadow_char(value: f32) -> char {
    let gray_scale = [
        '.', '¨', '-', '"', '°', '+', '=', 'c', 'o', '*', '#', '%', '@',
    ];
    let length = gray_scale.len() as f32;
    let index = (value * length).floor() as usize;
    if index >= length as usize {
        '@'
    } else {
        gray_scale[index]
    }
}

pub fn dot_product(vector_1: &[f32; 3], vector_2: &[f32; 3]) -> f32 {
    let [x1, y1, z1] = vector_1;
    let [x2, y2, z2] = vector_2;
    (x1 * x2) + (y1 * y2) + (z1 * z2)
}

fn vector_norm(vector: &[f32; 3]) -> f32 {
    let x = vector[0];
    let y = vector[1];
    let z = vector[2];
    (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
}

pub fn calculate_shadow_value(light: &[f32; 3], normal: &[f32; 3]) -> f32 {
    let light_norm = vector_norm(light);
    let normal_norm = vector_norm(normal);
    let dot_product = dot_product(light, normal);
    -(dot_product / (light_norm * normal_norm))
}

pub fn is_face_visible(normal: &[f32; 3], observer: &[f32; 3]) -> bool {
    let dot_product = dot_product(normal, observer);
    dot_product < 0.0
}

pub fn calculate_mean_z(vertices: &[[f32; 3]; 3]) -> f32 {
    let [v1, v2, v3] = vertices;
    let z1 = v1[2];
    let z2 = v2[2];
    let z3 = v3[2];
    (z1 + z2 + z3) / 3.0
}

fn cross_product_2d(vector_1: &[f32; 2], vector_2: &[f32; 2]) -> f32 {
    let [x1, y1] = vector_1;
    let [x2, y2] = vector_2;
    (x1 * y2) - (x2 * y1)
}

fn vector_subtract_2d(vector_1: &[f32; 2], vector_2: &[f32; 2]) -> [f32; 2] {
    let [x1, y1] = vector_1;
    let [x2, y2] = vector_2;
    [x1 - x2, y1 - y2]
}

pub fn is_point_inside_triangle(
    point: &[f32; 2],
    triangle: &[[f32; 2]; 3],
    triangle_area: &f32,
) -> bool {
    let [p1, p2, p3] = triangle;

    let area_1 = calculate_triangle_area(&[*p1, *p2, *point]);
    let area_2 = calculate_triangle_area(&[*p2, *p3, *point]);
    let area_3 = calculate_triangle_area(&[*p3, *p1, *point]);

    let sum = area_1 + area_2 + area_3;

    &sum < triangle_area || (sum - triangle_area).abs() < 0.001
}

pub fn calculate_triangle_area(triangle: &[[f32; 2]; 3]) -> f32 {
    let [p1, p2, p3] = triangle;
    let a = vector_subtract_2d(p1, p2);
    let b = vector_subtract_2d(p1, p3);

    let cross_product = cross_product_2d(&a, &b);

    cross_product.abs() / 2.0
}
