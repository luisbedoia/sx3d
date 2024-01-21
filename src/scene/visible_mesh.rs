use crate::utils::{calculate_mean_z, calculate_shadow_value, is_face_visible};
use std::{collections::HashMap, f32::INFINITY};
use stl_io::{IndexedMesh, Vertex};

#[derive(Debug)]
pub struct VisibleFace {
    vertices: [usize; 3],
    normal: [f32; 3],
    shadow_value: f32,
    mean_z: f32,
}

impl VisibleFace {
    pub fn new(vertices: [usize; 3], normal: [f32; 3]) -> VisibleFace {
        VisibleFace {
            vertices,
            normal,
            shadow_value: 0.1,
            mean_z: 0.0,
        }
    }

    pub fn get_vertices(&self) -> &[usize; 3] {
        &self.vertices
    }

    pub fn get_normal(&self) -> &[f32; 3] {
        &self.normal
    }

    pub fn get_shadow_value(&self) -> &f32 {
        &self.shadow_value
    }

    pub fn set_shadow_value(&mut self, shadow_value: f32) {
        self.shadow_value = shadow_value;
    }

    pub fn get_mean_z(&self) -> &f32 {
        &self.mean_z
    }

    pub fn set_mean_z(&mut self, mean_z: &f32) {
        self.mean_z = *mean_z;
    }
}

#[derive(Debug)]
pub struct VisibleMesh {
    vertices: HashMap<usize, [f32; 3]>,
    faces: Vec<VisibleFace>,
}

impl VisibleMesh {
    pub fn new(mesh: &IndexedMesh, light: &[f32; 3], observer: &[f32; 3]) -> VisibleMesh {
        let mut visible_mesh = VisibleMesh {
            vertices: HashMap::new(),
            faces: Vec::new(),
        };

        let original_vertices = &mesh.vertices;
        let original_faces = &mesh.faces;

        for face in original_faces.iter() {
            let vertices = face.vertices;
            let normal: [f32; 3] = face.normal.into();

            if is_face_visible(&normal, observer) {
                let shadow_value = calculate_shadow_value(light, &normal);
                let mut visible_face = VisibleFace::new(vertices, normal);
                let original_vertex_1: [f32; 3] = original_vertices[vertices[0]].into();
                let original_vertex_2: [f32; 3] = original_vertices[vertices[1]].into();
                let original_vertex_3: [f32; 3] = original_vertices[vertices[2]].into();
                let mean_z =
                    calculate_mean_z(&[original_vertex_1, original_vertex_2, original_vertex_3]);
                visible_face.set_shadow_value(shadow_value);
                visible_face.set_mean_z(&mean_z);
                visible_mesh.set_face(visible_face, original_vertices);
            }
        }

        visible_mesh
    }

    pub fn new_from_vertices_and_visible_faces(
        vertices: HashMap<usize, [f32; 3]>,
        faces: Vec<VisibleFace>,
    ) -> VisibleMesh {
        VisibleMesh { vertices, faces }
    }

    pub fn get_faces(&self) -> &Vec<VisibleFace> {
        &self.faces
    }

    pub fn get_face_vertices_xy(&self, face_index: usize) -> [[f32; 2]; 3] {
        let face = &self.faces[face_index];
        let vertices = face.get_vertices();

        let vertex_1 = self.vertices[&vertices[0]];
        let vertex_2 = self.vertices[&vertices[1]];
        let vertex_3 = self.vertices[&vertices[2]];

        let vertex_1_2d = [vertex_1[0], vertex_1[1]];
        let vertex_2_2d = [vertex_2[0], vertex_2[1]];
        let vertex_3_2d = [vertex_3[0], vertex_3[1]];

        [vertex_1_2d, vertex_2_2d, vertex_3_2d]
    }

    pub fn get_face_bounding_box_xy(&self, face_index: usize) -> [[f32; 2]; 2] {
        let face_vertices = self.get_face_vertices_xy(face_index);
        let mut min_x = INFINITY;
        let mut min_y = INFINITY;
        let mut max_x = -INFINITY;
        let mut max_y = -INFINITY;

        for vertex in face_vertices.iter() {
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

        [[min_x, min_y], [max_x, max_y]]
    }

    pub fn get_vertices(&self) -> &HashMap<usize, [f32; 3]> {
        &self.vertices
    }

    fn set_face(&mut self, face: VisibleFace, original_vertices: &[Vertex]) {
        let vertices = face.get_vertices();
        for vertex_index in vertices.iter() {
            let vertex = original_vertices[*vertex_index];
            self.set_vertex(vertex_index, &[vertex[0], vertex[1], vertex[2]]);
        }

        self.faces.push(face);
    }

    fn set_vertex(&mut self, index: &usize, vertex: &[f32; 3]) {
        if !self.vertex_exists(index) {
            self.vertices.insert(*index, *vertex);
        }
    }

    fn vertex_exists(&self, index: &usize) -> bool {
        self.vertices.contains_key(index)
    }
}
