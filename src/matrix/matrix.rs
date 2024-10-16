use crate::scene::VisibleMesh;
use crate::utils::{calculate_triangle_area, get_shadow_char, is_point_inside_triangle};
use std::char;
use std::f32::INFINITY;
use std::ops::Range;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Matrix {
    cols: usize,
    rows: usize,
    col_min: usize,
    row_min: usize,
    maximum_diameter: f32,
    chars_per_row: usize,
    data_length: usize,
    data: Arc<Mutex<Vec<char>>>,
    mean_z: Arc<Mutex<Vec<f32>>>,
}

impl Matrix {
    pub fn new(cols: usize, maximum_diameter: f32) -> Matrix {
        if cols % 2 == 0 {
            panic!("cols must be odd");
        }

        let rows = cols;
        let chars_per_row = 3 * cols - 2;
        let data_length = chars_per_row * rows;
        let col_min = (cols - 1) / 2;
        let row_min = (rows - 1) / 2;

        Matrix {
            cols,
            rows: cols,
            col_min,
            row_min,
            maximum_diameter,
            chars_per_row,
            data_length,
            data: Arc::new(Mutex::new(vec![' '; data_length])),
            mean_z: Arc::new(Mutex::new(vec![-INFINITY; data_length])),
        }
    }
    pub fn get_frame(&self) -> String {
        let mut frame = String::with_capacity(self.data_length + 2 * self.rows);
        let data = self.get_data();

        let mut line = String::with_capacity(self.chars_per_row);
        for (i, char) in data.iter().enumerate() {
            line.push(*char);

            if (i + 1) % self.chars_per_row == 0 {
                line = line.chars().rev().collect();
                line.push('\r');
                line.push('\n');
                frame.push_str(&line);
                line.clear();
            }
        }

        frame
    }

    pub fn project(&self, visible_mesh: &VisibleMesh) {
        let faces = visible_mesh.get_faces();

        self.data.lock().unwrap().iter_mut().for_each(|char| {
            *char = ' ';
        });

        self.mean_z.lock().unwrap().iter_mut().for_each(|z| {
            *z = -INFINITY;
        });

        for (face_index, face) in faces.iter().enumerate() {
            let face_bounary_box = visible_mesh.get_face_bounding_box_xy(face_index);
            let face_vertices = visible_mesh.get_face_vertices_xy(face_index);
            let face_area = calculate_triangle_area(&face_vertices);
            let shadow_value = face.get_shadow_value();
            let mean_z = face.get_mean_z();
            let shadow_char = get_shadow_char(*shadow_value);

            let [min_point, max_point] = face_bounary_box;
            let (min_col, min_row) = self.coordinates_to_indexes(min_point[0], min_point[1]);
            let (max_col, max_row) = self.coordinates_to_indexes(max_point[0], max_point[1]);

            self.clone().scan_boundary_box(
                &face_vertices,
                &face_area,
                &shadow_char,
                mean_z,
                min_col,
                min_row,
                max_col,
                max_row,
            );
        }
    }

    fn get_data(&self) -> Vec<char> {
        let data_reversed = self.data.lock().unwrap().to_vec();

        data_reversed.into_iter().rev().collect()
    }

    fn coordinates_to_indexes(&self, x: f32, y: f32) -> (isize, isize) {
        let col = ((x / self.maximum_diameter) * (self.cols) as f32).ceil() as isize;
        let row = ((y / self.maximum_diameter) * (self.rows) as f32).ceil() as isize;
        (col, row)
    }

    fn indexes_to_coordinates(&self, col: isize, row: isize) -> [f32; 2] {
        let x = (col as f32 / (self.cols) as f32) * self.maximum_diameter;
        let y = (row as f32 / (self.rows) as f32) * self.maximum_diameter;
        [x, y]
    }

    fn get_index(&self, col: isize, row: isize) -> usize {
        let transported_col = (col + self.col_min as isize) as usize;
        let transported_row = (row + self.row_min as isize) as usize;

        if transported_col >= self.cols || transported_row >= self.rows {
            println!("col: {}", col);
            println!("row: {}", row);
            panic!("index out of bounds");
        }

        3 * transported_col + transported_row * self.chars_per_row
    }

    pub fn set(&mut self, col: isize, row: isize, shadow_char: char, mean_z: &f32) {
        let index = self.get_index(col, row);
        let current_mean_z = self.mean_z.lock().unwrap()[index];

        if *mean_z > current_mean_z {
            self.mean_z.lock().unwrap()[index] = *mean_z;
            self.data.lock().unwrap()[index] = shadow_char;
        }
    }

    fn scan_boundary_box(
        &mut self,
        face_vertices: &[[f32; 2]; 3],
        face_area: &f32,
        shadow_char: &char,
        mean_z: &f32,
        min_col: isize,
        min_row: isize,
        max_col: isize,
        max_row: isize,
    ) {
        let row_range = Self::get_asc_range(min_row, max_row);
        let col_range = Self::get_asc_range(min_col, max_col);

        for row in row_range {
            for col in col_range.clone() {
                let point = self.indexes_to_coordinates(col, row);
                let is_point_inside = is_point_inside_triangle(&point, face_vertices, face_area);

                if is_point_inside {
                    self.set(col, row, *shadow_char, mean_z);
                }
            }
        }
    }

    fn get_asc_range(number_1: isize, number_2: isize) -> Range<isize> {
        if number_1 < number_2 {
            number_1..(number_2 + 1)
        } else {
            number_2..(number_1 + 1)
        }
    }
}
