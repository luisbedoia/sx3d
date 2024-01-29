use stl_io::{IndexedMesh, IndexedTriangle, Vector, Vertex};

#[derive(Debug)]
pub struct ObjectAttributes {
    maximum_radius: f32,
}

impl ObjectAttributes {
    pub fn new(indexed_mesh: &IndexedMesh) -> ObjectAttributes {
        let maximum_radius: f32 = Self::calculate_maximum_radius(indexed_mesh);
        ObjectAttributes { maximum_radius }
    }

    pub fn get_maximum_radius(&self) -> &f32 {
        &self.maximum_radius
    }

    fn calculate_maximum_radius(mesh: &IndexedMesh) -> f32 {
        let mut maximum_radius: f32 = 0.0;
        for vertex in &mesh.vertices {
            let radius: f32 = Self::calculate_vertex_radius(vertex);
            if radius > maximum_radius {
                maximum_radius = radius;
            }
        }
        maximum_radius
    }

    fn calculate_vertex_radius(vertex: &Vector<f32>) -> f32 {
        let x = vertex[0];
        let y = vertex[1];
        let z = vertex[2];

        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }
}

#[derive(Debug)]
pub struct Object {
    attributes: ObjectAttributes,
    mesh: IndexedMesh,
}

impl Object {
    pub fn new(indexed_mesh: IndexedMesh) -> Object {
        let mesh = Object::move_mesh_to_center(indexed_mesh);
        let attributes = ObjectAttributes::new(&mesh);

        Object { attributes, mesh }
    }

    pub fn get_mesh(&self) -> &IndexedMesh {
        &self.mesh
    }

    pub fn rotate_delta_x(&mut self, delta_angle_x: &f32) {
        let angles = (delta_angle_x, &0.0_f32, &0.0_f32);
        self.rotate_mesh(angles);
    }

    pub fn rotate_delta_y(&mut self, delta_angle: &f32) {
        let angles = (&0.0_f32, delta_angle, &0.0_f32);
        self.rotate_mesh(angles);
    }

    pub fn rotate_delta_z(&mut self, delta_angle: &f32) {
        let angles = (&0.0_f32, &0.0_f32, delta_angle);
        self.rotate_mesh(angles);
    }

    pub fn get_maximum_radius(&self) -> &f32 {
        self.attributes.get_maximum_radius()
    }

    fn rotate_mesh(&mut self, angles: (&f32, &f32, &f32)) {
        let new_vertices = self.rotate_vertices(&self.mesh.vertices, angles);
        let new_faces = self.rotate_faces(&self.mesh.faces, angles);

        self.mesh = IndexedMesh {
            vertices: new_vertices,
            faces: new_faces,
        }
    }

    fn move_mesh_to_center(mesh: IndexedMesh) -> IndexedMesh {
        let new_vertices: Vec<Vector<f32>> = Object::move_vertices_to_center(mesh.vertices);

        let new_mesh: IndexedMesh = IndexedMesh {
            vertices: new_vertices,
            faces: mesh.faces.clone(),
        };
        new_mesh
    }

    fn calculate_center_coordinates(vertices: &Vec<Vector<f32>>) -> (f32, f32, f32) {
        let len: f32 = vertices.len() as f32;
        let mut x_sum: f32 = 0.0;
        let mut y_sum: f32 = 0.0;
        let mut z_sum: f32 = 0.0;

        for vertex in vertices {
            x_sum += vertex[0];
            y_sum += vertex[1];
            z_sum += vertex[2];
        }

        let x = x_sum / len;
        let y = y_sum / len;
        let z = z_sum / len;

        (x, y, z)
    }

    fn move_vertices_to_center(vertices: Vec<Vector<f32>>) -> Vec<Vector<f32>> {
        let (x, y, z) = Object::calculate_center_coordinates(&vertices);
        let mut new_vertices: Vec<Vector<f32>> = Vec::new();

        for vertex in vertices {
            new_vertices.push(Object::move_vertex_against_vector(vertex, (x, y, z)));
        }

        new_vertices
    }

    fn move_vertex_against_vector(vertex: Vector<f32>, vector: (f32, f32, f32)) -> Vector<f32> {
        let (vector_x, vector_y, vector_z) = vector;
        let x = vertex[0];
        let y = vertex[1];
        let z = vertex[2];

        Vertex::new([x - vector_x, y - vector_y, z - vector_z])
    }

    fn rotate_tuple_over_x(&self, tuple: (&mut f32, &mut f32, &mut f32), angle_x: &f32) {
        let (_x, y, z) = tuple;

        let cos = angle_x.cos();
        let sin = angle_x.sin();

        let new_y = *y * cos - *z * sin;
        let new_z = *y * sin + *z * cos;

        *y = new_y;
        *z = new_z;
    }

    fn rotate_tuple_over_y(&self, tuple: (&mut f32, &mut f32, &mut f32), angle_y: &f32) {
        let (x, _y, z) = tuple;

        let cos = angle_y.cos();
        let sin = angle_y.sin();

        let new_x = *x * cos + *z * sin;
        let new_z = -*x * sin + *z * cos;

        *x = new_x;
        *z = new_z;
    }

    fn rotate_tuple_over_z(&self, tuple: (&mut f32, &mut f32, &mut f32), angle_z: &f32) {
        let (x, y, _z) = tuple;

        let cos = angle_z.cos();
        let sin = angle_z.sin();

        let new_x = *x * cos - *y * sin;
        let new_y = *x * sin + *y * cos;

        *x = new_x;
        *y = new_y;
    }

    fn rotate_vertex(&self, vertex: &Vector<f32>, angles: (&f32, &f32, &f32)) -> Vector<f32> {
        let (angle_x, angle_y, angle_z) = angles;
        let mut x = vertex[0];
        let mut y = vertex[1];
        let mut z = vertex[2];

        if angle_x.abs() > 0.0_f32 {
            self.rotate_tuple_over_x((&mut x, &mut y, &mut z), angle_x);
        }

        if angle_y.abs() > 0.0_f32 {
            self.rotate_tuple_over_y((&mut x, &mut y, &mut z), angle_y);
        }

        if angle_z.abs() > 0.0f32 {
            self.rotate_tuple_over_z((&mut x, &mut y, &mut z), angle_z);
        }

        Vertex::new([x, y, z])
    }

    fn rotate_face(&self, face: &IndexedTriangle, angles: (&f32, &f32, &f32)) -> IndexedTriangle {
        let normal = self.rotate_vertex(&face.normal, angles);
        IndexedTriangle {
            vertices: face.vertices,
            normal,
        }
    }

    fn rotate_vertices(
        &self,
        vertices: &Vec<Vector<f32>>,
        angles: (&f32, &f32, &f32),
    ) -> Vec<Vector<f32>> {
        let mut new_vertices: Vec<Vector<f32>> = Vec::new();

        for vertex in vertices {
            new_vertices.push(self.rotate_vertex(vertex, angles));
        }

        new_vertices
    }

    fn rotate_faces(
        &self,
        faces: &Vec<IndexedTriangle>,
        angles: (&f32, &f32, &f32),
    ) -> Vec<IndexedTriangle> {
        let mut new_faces: Vec<IndexedTriangle> = Vec::new();

        for face in faces {
            new_faces.push(self.rotate_face(face, angles));
        }

        new_faces
    }
}
