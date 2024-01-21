# SX3D

SX3D is a straightforward 3D STL files viewer designed for the console. The application provides a convenient way to render and view STL files in a text-based environment.

## Usage

To get started, use the following command:

```bash
sx3d --help
```

This will display the available options and their usage:

```bash
Usage: sx3d [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>  Path to the STL file to render
  -c, --cols <COLS>  Number of columns to use for rendering in the console [default: 31]
  -h, --help         Print help
  -V, --version      Print version
```

## Installation

1. Clone the repository:

```bash
git clone https://github.com/luisbedoia/sx3d.git
```

2. Enter the SX3D directory:

```bash
cd sx3d
```

3. Install using Cargo:

```bash
cargo install --path .
```

Now, you can run `sx3d` from any location in your terminal.

## Examples

To test the viewer, navigate to the `examples` directory and run the following command:

```bash
sx3d -p cube_ascii.stl -c 41
```

This will visualize the 3D STL file using 41 rows in the console.

![](https://github.com/luisbedoia/sx3d/blob/main/examples/cube.gif)

## Uninstallation

If you ever need to uninstall SX3D, go to the SX3D directory and run:

```bash
cargo uninstall
```

This will remove the application from your system.

Feel free to explore and enjoy viewing 3D STL files in your console using SX3D!