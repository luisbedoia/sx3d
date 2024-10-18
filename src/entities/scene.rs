use super::Object;
use super::Vector3D;
use super::VisibleIndexedMesh3D;

pub struct Scene {
    object: Object,
    light: Vector3D,
    observer: Vector3D,
}

impl Scene {
    pub fn new(object: Object, light: Vector3D, observer: Vector3D) -> Scene {
        Scene {
            object,
            light,
            observer,
        }
    }

    pub fn rotate_delta_x(&mut self, delta_angle: &f32) {
        self.object.rotate_mesh((delta_angle, &0.0, &0.0));
    }

    pub fn rotate_delta_y(&mut self, delta_angle: &f32) {
        self.object.rotate_mesh((&0.0, delta_angle, &0.0));
    }

    pub fn rotate_delta_z(&mut self, delta_angle: &f32) {
        self.object.rotate_mesh((&0.0, &0.0, delta_angle));
    }

    pub fn get_visible_mesh(&self) -> VisibleIndexedMesh3D {
        let mesh = self.object.get_mesh();
        VisibleIndexedMesh3D::new(mesh, &self.light, &self.observer)
    }
}
