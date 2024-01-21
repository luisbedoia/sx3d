use std::io::{stdin, stdout, Write};
use std::thread;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::controller::Controller;
use crate::matrix::Matrix;
use crate::scene::scene::Scene;
use crate::scene::Object;

use super::read_mesh;

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

    pub fn start(&mut self, path: String, cols: usize) -> thread::JoinHandle<()> {
        let indexed_mesh = read_mesh(path).unwrap();
        let object = Object::new(indexed_mesh.clone());
        let maximum_diameter = 2.0 * object.get_maximum_radius();

        let scene = Scene::new(object, [-1.0, -1.0, -1.0], [0.0, 0.0, -1.0]);
        let matrix = Matrix::new(cols, maximum_diameter);

        let mut controller = Controller::new(scene, matrix);

        thread::spawn(move || {
            let stdin = stdin();
            let mut stdout = stdout().into_raw_mode().unwrap();

            Self::print_frame(
                "To Start, press enter. To rotate the object, use the arrow keys. To quit, press 'q'.".to_string(),
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
        })
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
