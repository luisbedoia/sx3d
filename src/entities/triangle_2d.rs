use super::{TriangleArea, Vector2D};

pub struct VisibleTriangle2D {
    pub vertices: [Vector2D; 3],
    pub shadow_value: f32,
    pub mean_z: f32,
    pub area: Option<f32>,
}

impl VisibleTriangle2D {
    pub fn get_bounding_box_2d(&self) -> [Vector2D; 2] {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for vertex in self.vertices.iter() {
            let x = vertex[0];
            let y = vertex[1];

            if x < min_x {
                min_x = x;
            }

            if y < min_y {
                min_y = y;
            }

            if x > max_x {
                max_x = x;
            }

            if y > max_y {
                max_y = y;
            }
        }

        [Vector2D::new(min_x, min_y), Vector2D::new(max_x, max_y)]
    }

    pub fn area(&self) -> f32 {
        self.vertices.calculate_area()
    }

    pub fn contains_point(&self, point: &Vector2D) -> bool {
        let [p1, p2, p3] = self.vertices;
        let area_1 = [p1, p2, *point].calculate_area();
        let area_2 = [p2, p3, *point].calculate_area();
        let area_3 = [p3, p1, *point].calculate_area();

        let sum = area_1 + area_2 + area_3;

        sum <= self.area() || (sum - self.area()).abs() < 0.001
    }
}
