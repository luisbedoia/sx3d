use super::{IndexedMesh3D, Triangle3D, Vector3D, VisibleIndexedTriangle3D, VisibleTriangle2D};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct VisibleIndexedMesh3D {
    pub vertices: HashMap<usize, Vector3D>,
    pub triangles: Vec<VisibleIndexedTriangle3D>,
}

pub struct VisibleIndexedMesh3DIterator<'a> {
    vertices: &'a HashMap<usize, Vector3D>,
    triangles: &'a Vec<VisibleIndexedTriangle3D>,
    index: usize,
}

impl VisibleIndexedMesh3D {
    pub fn new(
        mesh: &IndexedMesh3D,
        light: &Vector3D,
        observer: &Vector3D,
    ) -> VisibleIndexedMesh3D {
        let visible_mesh = Arc::new(Mutex::new(VisibleIndexedMesh3D {
            vertices: HashMap::new(),
            triangles: Vec::new(),
        }));

        mesh.iter().par_bridge().for_each(|triangle| {
            if triangle.is_visible(observer) {
                let shadow_value = triangle.shadow_value(light);
                let mean_z = triangle.mean_z();

                visible_mesh
                    .lock()
                    .unwrap()
                    .set_triangle(triangle, shadow_value, mean_z);
            }
        });

        let result = visible_mesh.lock().unwrap().clone();
        result
    }

    fn set_triangle(&mut self, triangle: Triangle3D, shadow_value: f32, mean_z: f32) {
        let vertices_indices = triangle.vertices_indices;
        let vertices = triangle.vertices;

        for (index, vertex) in vertices.iter().enumerate() {
            self.set_vertex(&vertices_indices[index], vertex);
        }

        let visible_triangle = VisibleIndexedTriangle3D {
            vertices_indices,
            normal: triangle.normal,
            shadow_value,
            mean_z,
        };

        self.triangles.push(visible_triangle);
    }

    fn set_vertex(&mut self, index: &usize, vertex: &Vector3D) {
        if !self.exists_vertex(index) {
            self.vertices.insert(*index, *vertex);
        }
    }

    fn exists_vertex(&self, index: &usize) -> bool {
        self.vertices.contains_key(index)
    }

    pub fn iter(&self) -> VisibleIndexedMesh3DIterator {
        VisibleIndexedMesh3DIterator {
            vertices: &self.vertices,
            triangles: &self.triangles,
            index: 0,
        }
    }
}

impl Iterator for VisibleIndexedMesh3DIterator<'_> {
    type Item = VisibleTriangle2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.triangles.len() {
            let visible_indexed_triangle = &self.triangles[self.index];
            let vertices = [
                self.vertices[&visible_indexed_triangle.vertices_indices[0]].into(),
                self.vertices[&visible_indexed_triangle.vertices_indices[1]].into(),
                self.vertices[&visible_indexed_triangle.vertices_indices[2]].into(),
            ];

            let visible_triangle = VisibleTriangle2D {
                vertices,
                shadow_value: visible_indexed_triangle.shadow_value,
                mean_z: visible_indexed_triangle.mean_z,
                area: None,
            };

            self.index += 1;

            Some(visible_triangle)
        } else {
            None
        }
    }
}
