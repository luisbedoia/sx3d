use super::Vector3D;

pub struct Triangle3D {
    pub normal: Vector3D,
    pub vertices: [Vector3D; 3],
    pub vertices_indices: [usize; 3],
}

impl Triangle3D {
    pub fn is_visible(&self, observer: &Vector3D) -> bool {
        self.normal.dot_product(observer) < 0.0
    }

    pub fn shadow_value(&self, light: &Vector3D) -> f32 {
        let light_norm = light.norm();
        let normal_norm = self.normal.norm();
        let dot_product = light.dot_product(&self.normal);
        -(dot_product / (light_norm * normal_norm))
    }

    pub fn mean_z(&self) -> f32 {
        let z0 = self.vertices[0][2];
        let z1 = self.vertices[1][2];
        let z2 = self.vertices[2][2];

        (z0 + z1 + z2) / 3.0
    }
}

#[derive(Debug, Clone)]
pub struct IndexedTriangle3D {
    pub normal: Vector3D,
    pub vertices_indices: [usize; 3],
}

impl IndexedTriangle3D {
    pub fn rotate(&mut self, angles: (&f32, &f32, &f32)) {
        self.normal.rotate(angles);
    }
}

#[derive(Clone)]
pub struct VisibleIndexedTriangle3D {
    pub vertices_indices: [usize; 3],
    pub normal: Vector3D,
    pub shadow_value: f32,
    pub mean_z: f32,
}
