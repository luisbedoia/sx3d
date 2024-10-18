use super::Vector3D;
use std::ops::{Index, IndexMut, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector2D(pub [f32; 2]);

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D([x, y])
    }

    pub fn cross_product(&self, other: &Vector2D) -> f32 {
        let Vector2D([x1, y1]) = self;
        let Vector2D([x2, y2]) = other;

        (x1 * y2) - (y1 * x2)
    }
}

impl From<Vector3D> for Vector2D {
    fn from(val: Vector3D) -> Self {
        let Vector3D([x, y, _]) = val;
        Vector2D([x, y])
    }
}

impl Index<usize> for Vector2D {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        let Vector2D(data) = self;
        &data[index]
    }
}

impl IndexMut<usize> for Vector2D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let Vector2D(data) = self;
        &mut data[index]
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        let Vector2D([x1, y1]) = self;
        let Vector2D([x2, y2]) = other;

        Vector2D([x1 - x2, y1 - y2])
    }
}

pub trait TriangleArea {
    fn calculate_area(&self) -> f32;
}

impl TriangleArea for [Vector2D; 3] {
    fn calculate_area(&self) -> f32 {
        let [p1, p2, p3] = self;
        let a = *p1 - *p2;
        let b = *p1 - *p3;

        let cross_product = a.cross_product(&b);

        cross_product.abs() / 2.0
    }
}
