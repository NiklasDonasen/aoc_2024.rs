use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use relative_path::RelativePath;

fn get_reader(path: PathBuf) -> BufReader<File> {
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    BufReader::new(file)

}

fn main() -> std::io::Result<()> {
    // Reading the input file
    let root: PathBuf = current_dir()?;
    let relative_path: &RelativePath = RelativePath::new("day_1/input.txt");
    let path: PathBuf = relative_path.to_path(root);
    let reader: BufReader<File> = get_reader(path);

    // Working on the input
    for line in reader.lines() {
        println!("{}", line?);
    }

    // Necessary to be able to use the `?`-magic
    Ok(())

}