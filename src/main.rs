use sx3d::input_output::Console;

fn main() {
    let path = "./examples/twisted_torus.stl".to_string();

    let mut keyboard_input = Console::new();
    let handle = keyboard_input.start(path);
    handle.join().unwrap();
}
