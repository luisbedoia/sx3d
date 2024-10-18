use super::read_mesh;
use super::Controller;
use crate::canvas::SquaredCanvas;
use crate::entities::object::Object;
use crate::entities::scene::Scene;
use crate::Vector3D;
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

pub struct Console {}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&mut self, path: String) {
        let indexed_mesh = read_mesh(path).unwrap();
        let object = Object::new(indexed_mesh.clone());
        let size = terminal_size().unwrap();

        let (mut i, j) = size;
        i = (i + 2) / 3;
        let mut cols: usize;

        if i >= j {
            cols = j as usize;
        } else {
            cols = i as usize;
        }

        if cols % 2 == 0 {
            cols -= 1;
        } else {
            cols -= 2;
        }

        let maximum_diameter = 2.0 * object.get_maximum_radius();

        let scene = Scene::new(
            object,
            Vector3D::new(-1.0, -1.0, -1.0),
            Vector3D::new(0.0, 0.0, -1.0),
        );
        let matrix = SquaredCanvas::new(cols, maximum_diameter);

        let mut controller = Controller::new(scene, matrix);

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        Self::print_frame(
                "To start press Enter key.\r\nTo rotate the object around XY axis use arrow keys.\r\nTo quit, press 'q'.".to_string(),
                &mut stdout,
            );
        stdout.flush().unwrap();

        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char('q')) => {
                    Self::print_frame("Goodbye!\r\n".to_string(), &mut stdout);
                    break;
                }
                Event::Key(Key::Char('\n')) => {
                    let frame = controller.enter_key();
                    Self::print_frame(frame, &mut stdout);
                }
                Event::Key(Key::Up) => {
                    let frame = controller.up_key();
                    Self::print_frame(frame, &mut stdout);
                }
                Event::Key(Key::Down) => {
                    let frame = controller.down_key();
                    Self::print_frame(frame, &mut stdout);
                }
                Event::Key(Key::Left) => {
                    let frame = controller.left_key();
                    Self::print_frame(frame, &mut stdout);
                }
                Event::Key(Key::Right) => {
                    let frame = controller.right_key();
                    Self::print_frame(frame, &mut stdout);
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }

    fn print_frame(frame: String, stdout: &mut RawTerminal<std::io::Stdout>) {
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            frame
        )
        .unwrap();
    }
}
