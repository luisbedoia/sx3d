use std::fs::{File, OpenOptions};
use stl_io::{read_stl, IndexedMesh};

pub fn read_mesh(path: String) -> Result<IndexedMesh, String> {
    let mut file = read_file(path)?;
    let mesh = read_mesh_from_file(&mut file)?;
    Ok(mesh)
}

fn read_file(path: String) -> Result<File, String> {
    let file_result = OpenOptions::new().read(true).open(&path);
    match file_result {
        Ok(file) => Ok(file),
        Err(error) => Err(format!("Error: {error} Reading File: {path}")),
    }
}

fn read_mesh_from_file(file: &mut File) -> Result<IndexedMesh, String> {
    let mesh_result = read_stl(file);
    match mesh_result {
        Ok(mesh) => Ok(mesh),
        Err(error) => Err(format!("Error: {error} Reading Mesh")),
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
