mod config;

use clap::{Arg, Command};
use std::env;
use std::path::Path;
use walkdir::WalkDir;
use crate::config::ignored_directories;

fn main() {
    let matches = Command::new("pathy")
        .version("1.0.0")
        .author("fenix <fenix@fearandesire.com>")
        .about("Displays the file tree of a directory")
        .arg(
            Arg::new("DIRECTORY")
                .help("The directory to display the file tree for (default: current directory)")
                .index(1),
        )
        .arg(
            Arg::new("ignore")
                .help("Directories or files to ignore")
                .short('i')
                .long("ignore")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let directory = matches
        .get_one::<String>("DIRECTORY")
        .map(|s| s.as_str())
        .unwrap_or(".");

    let mut ignore_dirs: Vec<String> = matches
        .get_many::<String>("ignore")
        .map(|values| values.cloned().collect())
        .unwrap_or_default();

    ignore_dirs.extend(ignored_directories());
    let path = if directory == "." {
        env::current_dir().expect("Failed to get current directory")
    } else {
        Path::new(directory).to_path_buf()
    };

    if path.is_dir() {
        print_file_tree(&path, 0, &ignore_dirs);
    } else {
        eprintln!("Error: {} is not a directory", path.display());
    }
}

fn pretty_print(prefix: &str, is_last: bool, name: &str) {
    let connector = if is_last { "└" } else { "├" };
    let icon = if name.contains('.') { "✱" } else { "📂" };
    println!("{}{} {} {}", prefix, connector, icon, name);
}

fn print_file_tree(dir: &Path, depth: usize, ignore_dirs: &[String]) {
    let prefix = " ".repeat(depth * 2);
    let entries: Vec<_> = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if ignore_dirs.iter().any(|ignore| ignore == file_name) {
            continue;
        }

        let is_last = i == entries.len() - 1;
        pretty_print(&prefix, is_last, file_name);

        if path.is_dir() {
            print_file_tree(path, depth + 1, ignore_dirs);
        }
    }
}