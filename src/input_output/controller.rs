use crate::{Scene, SquaredCanvas};
use std::f32::consts::PI;

const DEFAULT_DELTA: f32 = PI / 10.0;

pub struct Controller {
    pub scene: Scene,
    pub matrix: SquaredCanvas,
}

impl Controller {
    pub fn new(scene: Scene, matrix: SquaredCanvas) -> Controller {
        Controller { scene, matrix }
    }

    fn get_frame(&mut self) -> String {
        let visible_mesh = self.scene.get_visible_mesh();
        self.matrix.set_mesh(&visible_mesh);
        self.matrix.get_frame()
    }

    pub fn enter_key(&mut self) -> String {
        self.get_frame()
    }

    pub fn up_key(&mut self) -> String {
        self.scene.rotate_delta_x(&-DEFAULT_DELTA);
        self.get_frame()
    }

    pub fn down_key(&mut self) -> String {
        self.scene.rotate_delta_x(&DEFAULT_DELTA);
        self.get_frame()
    }

    pub fn left_key(&mut self) -> String {
        self.scene.rotate_delta_y(&-DEFAULT_DELTA);
        self.get_frame()
    }

    pub fn right_key(&mut self) -> String {
        self.scene.rotate_delta_y(&DEFAULT_DELTA);
        self.get_frame()
    }
}
