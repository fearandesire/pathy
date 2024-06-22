# pathy
Minimal Rust-based CLI tool to display directory structures

![pathy example screenshot](https://i.imgur.com/xiBe1yp.png)

## Features
- 🧠 **Smart Display**: Automatically excludes commonly ignored directories (like `.git` and `node_modules`) for a cleaner tree visualization.
- 👨‍🦯 **Easy Exclusions**: Use the `-i` option to seamlessly omit any specific directories & files from the display.


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
- `<directory>`: Directory to display. _Defaults to the current directory._
- `-i`, or `--ignore`: Specify directories or files to ignore`
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
pathy -i docs node_modules .
# Ignore a directory and a any file 
pathy -i docs README.md
```
> **Notice:** Ignoring a directory / a file will explicitly exclude any exact match. E.g, ignoring `-i README.md` will ignore `README.md` from the file tree visualization entirely, no matter how far down it is.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to improve the functionality or documentation of pathy.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author
- [fenix](https://github.com/fearandesire)
