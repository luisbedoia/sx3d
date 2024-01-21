use crate::scene::error::Sx3dError;
use std::fs::{File, OpenOptions};
use stl_io::{read_stl, IndexedMesh};

pub fn read_mesh(path: String) -> Result<IndexedMesh, Sx3dError> {
    let mut file = read_file(path)?;
    let mesh = read_mesh_from_file(&mut file)?;
    Ok(mesh)
}

fn read_file(path: String) -> Result<File, Sx3dError> {
    let file_result = OpenOptions::new().read(true).open(&path);
    match file_result {
        Ok(file) => Ok(file),
        Err(error) => Err(Sx3dError {
            message: format!("Error Reading File: {path}"),
            error: Some(Box::new(error)),
        }),
    }
}

fn read_mesh_from_file(file: &mut File) -> Result<IndexedMesh, Sx3dError> {
    let mesh_result = read_stl(file);
    match mesh_result {
        Ok(mesh) => match mesh.validate() {
            Ok(_) => Ok(mesh),
            Err(error) => Err(Sx3dError {
                message: format!("Error Validating Mesh: {error}"),
                error: Some(Box::new(error)),
            }),
        },
        Err(error) => Err(Sx3dError {
            message: format!("Error Reading Mesh: {error}"),
            error: Some(Box::new(error)),
        }),
    }
}

#[cfg(test)]
mod read_tests {
    use super::*;

    #[test]
    fn it_should_read_ascii_stl() {
        let result = read_mesh("examples/cube_ascii.stl".to_string()).unwrap();
        assert_eq!(result.vertices.len(), 8);
        assert_eq!(result.faces.len(), 12);
        println!("{:?}", result.vertices[0]);
    }

    #[test]
    fn it_should_read_binary_stl() {
        let result = read_mesh("examples/cube_binary.stl".to_string()).unwrap();
        assert_eq!(result.vertices.len(), 8);
        assert_eq!(result.faces.len(), 12);
    }
}
