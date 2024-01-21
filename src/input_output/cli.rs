use clap::Parser;

/// A simple 3D STL files viewer on console
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the STL file to render
    #[arg(short, long)]
    path: String,

    /// Number of colums to use for rendering in the console
    #[arg(short, long, default_value_t = 31)]
    cols: usize,
}

pub fn get_file_path() -> (String, usize) {
    let args = Args::parse();
    (args.path, args.cols)
}
