# SX3D
A simple 3D STL files viewer on console. `sx3d --help`:
```
Usage: sx3d [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>  Path to the STL file to render
  -c, --cols <COLS>  Number of colums to use for rendering in the console [default: 31]
  -h, --help         Print help
  -V, --version      Print version
```

## Install
Clone the repo, enter the sx3d directory and run `cargo install --path .`, then you can run `sx3d` from anywhere.

## Examples
You can find some STL files to test in the `examples` directory. You can run `sx3d -p sx3d/examples/cube_ascii.stl -c 41` to visualize the 3D STL file.

## Uninstall
Enter the sx3d directory and run `cargo uninstall`. 
