use super::CalculateMaximumRadius;
use super::IndexedMesh3D;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Object {
    mesh: IndexedMesh3D,
    maximum_radius: f32,
}

impl Object {
    pub fn new(mut mesh: IndexedMesh3D) -> Object {
        mesh.move_to_center();
        let maximum_radius = mesh.vertices.calculate_maximum_radius();

        Object {
            mesh,
            maximum_radius,
        }
    }

    pub fn get_mesh(&self) -> &IndexedMesh3D {
        &self.mesh
    }

    pub fn get_maximum_radius(&self) -> &f32 {
        &self.maximum_radius
    }

    pub fn rotate_mesh(&mut self, angles: (&f32, &f32, &f32)) {
        self.mesh
            .vertices
            .par_iter_mut()
            .for_each(|vertex| vertex.rotate(angles));

        self.mesh.triangles.par_iter_mut().for_each(|triangle| {
            triangle.rotate(angles);
        });
    }
}
