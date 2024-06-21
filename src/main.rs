use clap::{Arg, ArgAction, Command};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("Ftree")
        .version("1.0")
        .author("fenix <fenix@fearandesire.com>")
        .about("Displays the file tree of a directory")
        .arg(
            Arg::new("DIRECTORY")
                .help("The directory to display the file tree for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("ignore")
                .help("Directories to ignore")
                .short('i')
                .long("ignore")
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let directory = matches.get_one::<String>("DIRECTORY").unwrap();

    let mut ignore_dirs: Vec<String> = matches
        .get_many::<String>("ignore")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    ignore_dirs.extend_from_slice(&[
        ".git".to_string(),
        ".idea".to_string(),
        "node_modules".to_string(),
        "target".to_string(),
        "dist".to_string(),
        ".vscode".to_string(),
        ".npmrc".to_string(),
    ]);

    let path = Path::new(directory);

    if path.is_dir() {
        print_file_tree(path, 0, &ignore_dirs);
    } else {
        eprintln!("Error: {} is not a directory", directory);
    }
}

fn pretty_print(prefix: &str, is_last: bool, name: &str) {
    let connector = if is_last { "└" } else { "├" };
    let icon = if name.contains('.') { "📜" } else { "📂" };
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
