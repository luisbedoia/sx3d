use rayon::prelude::*;
use std::ops::{Index, IndexMut, Sub};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub struct Vector3D(pub [f32; 3]);

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D([x, y, z])
    }
}

impl Vector3D {
    pub fn norm(&self) -> f32 {
        let Vector3D([x, y, z]) = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn dot_product(&self, other: &Vector3D) -> f32 {
        let Vector3D([x1, y1, z1]) = self;
        let Vector3D([x2, y2, z2]) = other;

        (x1 * x2) + (y1 * y2) + (z1 * z2)
    }

    pub fn cross_product(&self, other: &Vector3D) -> Vector3D {
        let Vector3D([x1, y1, z1]) = self;
        let Vector3D([x2, y2, z2]) = other;

        Vector3D([
            (y1 * z2) - (z1 * y2),
            (z1 * x2) - (x1 * z2),
            (x1 * y2) - (y1 * x2),
        ])
    }

    pub fn move_against_vector(&mut self, vector: Vector3D) {
        let Vector3D([x, y, z]) = self;
        let Vector3D([vector_x, vector_y, vector_z]) = vector;

        *x -= vector_x;
        *y -= vector_y;
        *z -= vector_z;
    }

    pub fn rotate(&mut self, angles: (&f32, &f32, &f32)) {
        let (angle_x, angle_y, angle_z) = angles;

        if angle_x.abs() > 0.0_f32 {
            self.rotate_over_x(angle_x);
        }

        if angle_y.abs() > 0.0_f32 {
            self.rotate_over_y(angle_y);
        }

        if angle_z.abs() > 0.0f32 {
            self.rotate_over_z(angle_z);
        }
    }

    fn rotate_over_x(&mut self, angle_x: &f32) {
        let y = self[1];
        let z = self[2];

        let cos = angle_x.cos();
        let sin = angle_x.sin();

        let new_y = y * cos - z * sin;
        let new_z = y * sin + z * cos;

        self[1] = new_y;
        self[2] = new_z;
    }

    fn rotate_over_y(&mut self, angle_y: &f32) {
        let x = self[0];
        let z = self[2];

        let cos = angle_y.cos();
        let sin = angle_y.sin();

        let new_x = x * cos + z * sin;
        let new_z = -x * sin + z * cos;

        self[0] = new_x;
        self[2] = new_z;
    }

    fn rotate_over_z(&mut self, angle_z: &f32) {
        let x = self[0];
        let y = self[1];

        let cos = angle_z.cos();
        let sin = angle_z.sin();

        let new_x = x * cos - y * sin;
        let new_y = x * sin + y * cos;

        self[0] = new_x;
        self[1] = new_y;
    }
}

impl Index<usize> for Vector3D {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        let Vector3D(data) = self;
        &data[index]
    }
}

impl IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let Vector3D(data) = self;
        &mut data[index]
    }
}

pub trait CalculateCenter {
    fn calculate_center(&self) -> Vector3D;
}

impl CalculateCenter for [Vector3D] {
    fn calculate_center(&self) -> Vector3D {
        let len = self.len() as f32;
        let x_sum = Arc::new(Mutex::new(0.0));
        let y_sum = Arc::new(Mutex::new(0.0));
        let z_sum = Arc::new(Mutex::new(0.0));

        self.par_iter().for_each(|vertex| {
            let mut x_sum = x_sum.lock().unwrap();
            let mut y_sum = y_sum.lock().unwrap();
            let mut z_sum = z_sum.lock().unwrap();

            *x_sum += vertex[0];
            *y_sum += vertex[1];
            *z_sum += vertex[2];
        });

        let x = *x_sum.lock().unwrap() / len;
        let y = *y_sum.lock().unwrap() / len;
        let z = *z_sum.lock().unwrap() / len;

        Vector3D::new(x, y, z)
    }
}

pub trait CalculateMaximumRadius {
    fn calculate_maximum_radius(&self) -> f32;
}

impl CalculateMaximumRadius for [Vector3D] {
    fn calculate_maximum_radius(&self) -> f32 {
        self.par_iter().map(|vertex| vertex.norm()).reduce(
            || 0.0,
            |a, b| {
                if a > b {
                    a
                } else {
                    b
                }
            },
        )
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        let Vector3D([x1, y1, z1]) = self;
        let Vector3D([x2, y2, z2]) = other;

        Vector3D([x1 - x2, y1 - y2, z1 - z2])
    }
}

pub trait CalculateNormal {
    fn calculate_normal(&self) -> Vector3D;
}

impl CalculateNormal for [Vector3D; 3] {
    fn calculate_normal(&self) -> Vector3D {
        let v1 = self[1] - self[0];
        let v2 = self[2] - self[0];

        let normal = v1.cross_product(&v2);
        let norm = normal.norm();

        Vector3D::new(normal[0] / norm, normal[1] / norm, normal[2] / norm)
    }
}
