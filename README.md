# [pathy](https://github.com/fearandesire/pathy) 🦀
Minimal Rust CLI tool to display directory structures

![version badge](https://img.shields.io/crates/v/pathy?labelColor=%23e77823&color=%23002fff
)
![maintenance status badge: as-is](https://badgen.net/static/status/as-is/orange)
---
![pathy example screenshot](https://i.imgur.com/pkDS7bw.png)
## Features
- 🧠 **Smart Display**: Automatically excludes commonly [ignored directories](src/config.rs) (e.g `.git` and `node_modules`) for a cleaner tree visualization.
- 👨‍🦯 **Easy Exclusions**: Use the `-i` option to seamlessly omit any specific directories & files from the display.
## Install
### via Cargo
```sh
cargo install pathy
```
### via Source
```sh
git clone https://github.com/fearandesire/pathy.git
cd pathy
cargo build --release
cargo install .
```

# Usage
Basic usage: `pathy <directory>`
## Arguments
- `<directory>`: Directory to display. _Defaults to the current directory._
- `-i`, or `--ignore`: Specify directories or files to ignore`
- `-h`, or `--help`: Display this help message
- `-v`, or `--version`: Display the current version
## Default Ignored Directories
You can review the default ignored directories [here](src/config.rs)
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
