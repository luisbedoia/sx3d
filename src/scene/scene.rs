use super::Object;
use super::VisibleMesh;

pub type Light = [f32; 3];
pub type Observer = [f32; 3];

pub struct Scene {
    object: Object,
    light: Light,
    observer: Observer,
}

impl Scene {
    pub fn new(object: Object, light: Light, observer: Observer) -> Scene {
        Scene {
            object,
            light,
            observer,
        }
    }

    pub fn rotate_delta_x(&mut self, delta_angle: &f32) {
        self.object.rotate_delta_x(delta_angle);
    }

    pub fn rotate_delta_y(&mut self, delta_angle: &f32) {
        self.object.rotate_delta_y(delta_angle);
    }

    pub fn rotate_delta_z(&mut self, delta_angle: &f32) {
        self.object.rotate_delta_z(delta_angle);
    }

    pub fn get_visible_mesh(&self) -> VisibleMesh {
        let mesh = self.object.get_mesh();
        VisibleMesh::new(mesh, &self.light, &self.observer)
    }
}
