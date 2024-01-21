use std::io::{stdin, stdout, Write};
use std::thread;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::scene::scene::Scene;
use crate::scene::Object;
use crate::screen::Matrix;

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
        let matrix = Matrix::new(cols, maximum_diameter);

        let mut scene = Scene::new(object, [-1.0, -1.0, -1.0], [0.0, 0.0, -1.0]);
        thread::spawn(move || {
            let stdin = stdin();
            let mut stdout = stdout().into_raw_mode().unwrap();

            writeln!(
                stdout,
                "{}{}press q to quit!",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )
            .unwrap();
            stdout.flush().unwrap();

            for c in stdin.events() {
                let evt = c.unwrap();
                match evt {
                    Event::Key(Key::Char('q')) => {
                        write!(stdout, "{}", termion::clear::All).unwrap();
                        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
                        write!(stdout, "Bye!").unwrap();
                        break;
                    }
                    Event::Key(Key::Up) => {
                        scene.rotate_delta_x(&-0.1);
                        let visible_mesh = scene.get_visible_mesh();
                        matrix.project(&visible_mesh);
                        let data = matrix.get_frame();

                        write!(
                            stdout,
                            "{}{}{}",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1),
                            data
                        )
                        .unwrap();
                    }
                    Event::Key(Key::Down) => {
                        scene.rotate_delta_x(&0.1);
                        let visible_mesh = scene.get_visible_mesh();
                        matrix.project(&visible_mesh);
                        let data = matrix.get_frame();

                        write!(
                            stdout,
                            "{}{}{}",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1),
                            data
                        )
                        .unwrap();
                    }
                    Event::Key(Key::Left) => {
                        scene.rotate_delta_y(&-0.1);
                        let visible_mesh = scene.get_visible_mesh();
                        matrix.project(&visible_mesh);
                        let data = matrix.get_frame();

                        write!(
                            stdout,
                            "{}{}{}",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1),
                            data
                        )
                        .unwrap();
                    }
                    Event::Key(Key::Right) => {
                        scene.rotate_delta_y(&0.1);
                        let visible_mesh = scene.get_visible_mesh();
                        matrix.project(&visible_mesh);
                        let data = matrix.get_frame();

                        write!(
                            stdout,
                            "{}{}{}",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1),
                            data
                        )
                        .unwrap();
                    }
                    _ => {}
                }
                stdout.flush().unwrap();
            }
        })
    }
}
