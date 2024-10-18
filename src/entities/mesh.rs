use super::{CalculateCenter, IndexedTriangle3D, Triangle3D, Vector3D};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct IndexedMesh3D {
    pub vertices: Vec<Vector3D>,
    pub triangles: Vec<IndexedTriangle3D>,
}

pub struct IndexedMesh3DIterator<'a> {
    vertices: &'a Vec<Vector3D>,
    triangles: &'a Vec<IndexedTriangle3D>,
    index: usize,
}

impl IndexedMesh3D {
    pub fn iter(&self) -> IndexedMesh3DIterator {
        IndexedMesh3DIterator {
            vertices: &self.vertices,
            triangles: &self.triangles,
            index: 0,
        }
    }

    pub fn move_to_center(&mut self) {
        let center = self.vertices.calculate_center();

        self.vertices
            .par_iter_mut()
            .for_each(|vertex| vertex.move_against_vector(center));
    }
}

impl<'a> Iterator for IndexedMesh3DIterator<'a> {
    type Item = Triangle3D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.triangles.len() {
            let indexed_triangle = &self.triangles[self.index];
            let vertices = [
                self.vertices[indexed_triangle.vertices_indices[0]],
                self.vertices[indexed_triangle.vertices_indices[1]],
                self.vertices[indexed_triangle.vertices_indices[2]],
            ];
            let vertices_indices = indexed_triangle.vertices_indices;

            let triangle = Triangle3D {
                normal: indexed_triangle.normal,
                vertices,
                vertices_indices,
            };

            self.index += 1;

            Some(triangle)
        } else {
            None
        }
    }
}
