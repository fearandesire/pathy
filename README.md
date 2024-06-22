# pathy
Minimal Rust-based CLI tool to display directory structures

![pathy example screenshot](https://i.imgur.com/xiBe1yp.png)

## Features
- 🧠 **Smart Display**: Automatically excludes commonly ignored directories (like `.git` and `node_modules`) for a cleaner tree visualization.
- 👨‍🦯 **Easy Exclusions**: Use the `--ignore` option to seamlessly omit any specific directories from the display.


## Install
### via Cargo
### via Source
```sh
git clone https://github.com/yourusername/pathy.git
cd pathy
cargo build --release
```

# Usage
Basic usage: `pathy <directory>`
## Arguments
- `<directory>`: Directory to display
- `-i`, or `--ignore`: Specify a directory to ignore. Each directory must be specified with `-i`
## Default Ignored Directories
By default, pathy will ignore the following directories:
```
.git
.idea
node_modules
target
dist
.vscode
.npmrc
```

## Examples
```sh
pathy .
# Ignoring a directory
pathy -i docs
# Ignoring multiple directories
pathy -i docs -i node_modules .
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to improve the functionality or documentation of pathy.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author
- [fenix](https://github.com/fearandesire)
