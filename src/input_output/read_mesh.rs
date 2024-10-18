use std::fs::{File, OpenOptions};
use std::io::Read;
use stl_io::{read_stl, IndexedMesh};
use wavefront_obj::obj::parse;

use crate::{CalculateNormal, IndexedMesh3D, IndexedTriangle3D, Vector3D};

pub enum FileType {
    Stl,
    Obj,
}

pub fn get_file_type(path: String) -> Result<FileType, String> {
    let extension = path.split('.').last().unwrap();
    match extension {
        "stl" => Ok(FileType::Stl),
        "obj" => Ok(FileType::Obj),
        _ => Err(format!("Error: Unsupported File Type: {extension}")),
    }
}

pub fn read_mesh(path: String) -> Result<IndexedMesh3D, String> {
    let file_type = get_file_type(path.clone())?;
    let mut file = read_file(path)?;

    match file_type {
        FileType::Stl => {
            let stl_mesh = read_stl_mesh_from_file(&mut file)?;
            let mesh = convert_stl_to_indexed_mesh_3d(stl_mesh);
            Ok(mesh)
        }
        FileType::Obj => {
            let mut obj_data = String::new();
            file.read_to_string(&mut obj_data).unwrap();
            let obj_set = parse(obj_data).unwrap();
            let mesh = convert_obj_set_to_indexed_mesh_3d(obj_set);

            Ok(mesh)
        }
    }
}

fn read_file(path: String) -> Result<File, String> {
    let file_result = OpenOptions::new().read(true).open(&path);
    match file_result {
        Ok(file) => Ok(file),
        Err(error) => Err(format!("Error: {error} Reading File: {path}")),
    }
}

fn read_stl_mesh_from_file(file: &mut File) -> Result<IndexedMesh, String> {
    let mesh_result = read_stl(file);
    match mesh_result {
        Ok(mesh) => Ok(mesh),
        Err(error) => Err(format!("Error: {error} Reading Mesh")),
    }
}

fn convert_stl_to_indexed_mesh_3d(stl_mesh: stl_io::IndexedMesh) -> IndexedMesh3D {
    let vertices: Vec<Vector3D> = stl_mesh
        .vertices
        .iter()
        .map(|v| Vector3D::new(v[0], v[1], v[2]))
        .collect();

    let triangles = stl_mesh
        .faces
        .iter()
        .map(|f| {
            let vertices = [
                vertices[f.vertices[0]],
                vertices[f.vertices[1]],
                vertices[f.vertices[2]],
            ];
            let normal = vertices.calculate_normal();
            IndexedTriangle3D {
                normal,
                vertices_indices: [f.vertices[0], f.vertices[1], f.vertices[2]],
            }
        })
        .collect();

    IndexedMesh3D {
        vertices,
        triangles,
    }
}

fn convert_obj_set_to_indexed_mesh_3d(obj_set: wavefront_obj::obj::ObjSet) -> IndexedMesh3D {
    let mut mesh: IndexedMesh3D = IndexedMesh3D {
        vertices: Vec::new(),
        triangles: Vec::new(),
    };

    for object in obj_set.objects {
        if !object.geometry.is_empty() {
            convert_obj_to_indexed_mesh_3d(object, &mut mesh);
        }

        continue;
    }

    mesh
}

fn convert_obj_to_indexed_mesh_3d(obj: wavefront_obj::obj::Object, mesh: &mut IndexedMesh3D) {
    let last_index = mesh.vertices.len();

    let new_vertices: Vec<Vector3D> = obj
        .vertices
        .iter()
        .map(|v| Vector3D::new(v.x as f32, v.y as f32, v.z as f32))
        .collect();

    for geometry in obj.geometry {
        for shape in geometry.shapes {
            if let wavefront_obj::obj::Primitive::Triangle(a, b, c) = shape.primitive {
                let vertices = [new_vertices[a.0], new_vertices[b.0], new_vertices[c.0]];
                let normal = vertices.calculate_normal();
                let triangle = IndexedTriangle3D {
                    normal,
                    vertices_indices: [a.0 + last_index, b.0 + last_index, c.0 + last_index],
                };
                mesh.triangles.push(triangle);
            }
        }
    }

    mesh.vertices.extend(new_vertices);
}

#[cfg(test)]
mod read_tests {
    use super::*;

    #[test]
    fn it_should_read_ascii_stl() {
        let result = read_mesh("examples/cube_ascii.stl".to_string()).unwrap();
        assert_eq!(result.vertices.len(), 8);
        assert_eq!(result.triangles.len(), 12);
        println!("{:?}", result.vertices[0]);
    }

    #[test]
    fn it_should_read_binary_stl() {
        let result = read_mesh("examples/cube_binary.stl".to_string()).unwrap();
        assert_eq!(result.vertices.len(), 8);
        assert_eq!(result.triangles.len(), 12);
    }

    #[test]
    fn it_should_read_obj() {
        let _result = read_mesh("examples/cube.obj".to_string()).unwrap();
    }
}
