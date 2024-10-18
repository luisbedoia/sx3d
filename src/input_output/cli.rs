use clap::Parser;

/// A simple 3D files viewer on console
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the 3D file. It can be OBJ or STL.
    #[arg()]
    path: String,
}

pub fn get_file_path() -> String {
    let args = Args::parse();
    args.path
}
