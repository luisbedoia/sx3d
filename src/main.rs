use sx3d::input_output::{get_file_path, Console};

fn main() {
    let path = get_file_path();

    let mut console = Console::new();
    console.start(path);
}
