use crate::entities::VisibleIndexedMesh3D;
use crate::entities::{Vector2D, VisibleTriangle2D};
use std::char;
use std::ops::Range;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SquaredCanvas {
    pub cols: usize,
    pub zero_col: usize,
    pub maximum_diameter: f32,
    pub chars_per_row: usize,
    pub data_length: usize,
    pub data: Arc<Mutex<Vec<char>>>,
    pub mean_z: Arc<Mutex<Vec<f32>>>,
}

impl SquaredCanvas {
    pub fn new(cols: usize, maximum_diameter: f32) -> SquaredCanvas {
        if cols % 2 == 0 {
            panic!("cols must be odd");
        }

        let rows = cols;
        let chars_per_row = 3 * cols - 2;
        let data_length = chars_per_row * rows;
        let zero_col = (cols - 1) / 2;

        SquaredCanvas {
            cols,
            zero_col,
            maximum_diameter,
            chars_per_row,
            data_length,
            data: Arc::new(Mutex::new(vec![' '; data_length])),
            mean_z: Arc::new(Mutex::new(vec![-f32::INFINITY; data_length])),
        }
    }

    pub fn get_frame(&self) -> String {
        let mut frame = String::with_capacity(self.data_length + 2 * self.cols);
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

    pub fn set_mesh(&self, visible_mesh: &VisibleIndexedMesh3D) {
        self.data.lock().unwrap().iter_mut().for_each(|char| {
            *char = ' ';
        });

        self.mean_z.lock().unwrap().iter_mut().for_each(|z| {
            *z = -f32::INFINITY;
        });

        visible_mesh.iter().for_each(|triangle| {
            self.set_triangle(&triangle);
        });
    }

    fn get_data(&self) -> Vec<char> {
        let data_reversed = self.data.lock().unwrap().to_vec();

        data_reversed.into_iter().rev().collect()
    }

    fn get_index(&self, col: &isize, row: &isize) -> usize {
        let transported_col = (col + self.zero_col as isize) as usize;
        let transported_row = (row + self.zero_col as isize) as usize;

        if transported_col >= self.cols || transported_row >= self.cols {
            println!("col: {}", col);
            println!("row: {}", row);
            panic!("index out of bounds");
        }

        3 * transported_col + transported_row * self.chars_per_row
    }

    fn set_pixel(&self, col: &isize, row: &isize, shadow_char: &char, mean_z: &f32) {
        let index = self.get_index(col, row);
        let current_mean_z = self.mean_z.lock().unwrap()[index];

        if *mean_z > current_mean_z {
            self.mean_z.lock().unwrap()[index] = *mean_z;
            self.data.lock().unwrap()[index] = *shadow_char;
        }
    }

    fn coordinates_to_indexes(&self, x: &f32, y: &f32) -> (isize, isize) {
        let col = ((x / self.maximum_diameter) * (self.cols) as f32).ceil() as isize;
        let row = ((y / self.maximum_diameter) * (self.cols) as f32).ceil() as isize;
        (col, row)
    }

    fn indexes_to_coordinates(&self, col: isize, row: isize) -> Vector2D {
        let x = (col as f32 / (self.cols) as f32) * self.maximum_diameter;
        let y = (row as f32 / (self.cols) as f32) * self.maximum_diameter;
        Vector2D::new(x, y)
    }

    fn set_triangle(&self, triangle_2d: &VisibleTriangle2D) {
        let [min_point, max_point] = triangle_2d.get_bounding_box_2d();
        let shadow_value = triangle_2d.shadow_value;
        let shadow_char = get_shadow_char(shadow_value);
        let mean_z = triangle_2d.mean_z;
        let (min_col, min_row) = self.coordinates_to_indexes(&min_point[0], &min_point[1]);
        let (max_col, max_row) = self.coordinates_to_indexes(&max_point[0], &max_point[1]);
        let row_range = Self::get_asc_range(min_row, max_row);
        let col_range = Self::get_asc_range(min_col, max_col);

        for row in row_range {
            for col in col_range.clone() {
                let point = self.indexes_to_coordinates(col, row);
                let is_point_inside = triangle_2d.contains_point(&point);

                if is_point_inside {
                    self.set_pixel(&col, &row, &shadow_char, &mean_z);
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

pub fn get_shadow_char(value: f32) -> char {
    let gray_scale = [
        '.', ':', '-', '"', '+', '=', 'c', 'o', '*', '%', '#', 'M', '@',
    ];

    let length = gray_scale.len() as f32;
    let index = (value * length).floor() as usize;
    if index >= length as usize {
        '@'
    } else {
        gray_scale[index]
    }
}
