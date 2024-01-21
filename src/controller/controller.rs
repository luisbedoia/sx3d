use crate::{matrix::Matrix, scene::Scene};
use std::f32::consts::PI;

const DEFAULT_DELTA: f32 = PI / 10.0;

pub struct Controller {
    pub scene: Scene,
    pub matrix: Matrix,
}

impl Controller {
    pub fn new(scene: Scene, matrix: Matrix) -> Controller {
        Controller { scene, matrix }
    }

    fn get_frame(&self) -> String {
        self.matrix.get_frame()
    }

    pub fn enter_key(&mut self) -> String {
        let visible_mesh = self.scene.get_visible_mesh();
        self.matrix.project(&visible_mesh);
        self.get_frame()
    }

    pub fn up_key(&mut self) -> String {
        self.scene.rotate_delta_x(&-DEFAULT_DELTA);
        let visible_mesh = self.scene.get_visible_mesh();
        self.matrix.project(&visible_mesh);
        self.get_frame()
    }

    pub fn down_key(&mut self) -> String {
        self.scene.rotate_delta_x(&DEFAULT_DELTA);
        let visible_mesh = self.scene.get_visible_mesh();
        self.matrix.project(&visible_mesh);
        self.get_frame()
    }

    pub fn left_key(&mut self) -> String {
        self.scene.rotate_delta_y(&-DEFAULT_DELTA);
        let visible_mesh = self.scene.get_visible_mesh();
        self.matrix.project(&visible_mesh);
        self.get_frame()
    }

    pub fn right_key(&mut self) -> String {
        self.scene.rotate_delta_y(&DEFAULT_DELTA);
        let visible_mesh = self.scene.get_visible_mesh();
        self.matrix.project(&visible_mesh);
        self.get_frame()
    }
}
