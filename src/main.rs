use sx3d::input_output::{get_file_path, Console};

fn main() {
    let (path, cols) = get_file_path();

    let mut keyboard_input = Console::new();
    let handle = keyboard_input.start(path, cols);
    handle.join().unwrap();
}
