use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// use day_6::solution::solve_day_6;
// mod day_6;

fn get_reader(path: &PathBuf) -> BufReader<File> {
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    BufReader::new(file)
}

fn main() -> anyhow::Result<()> {
    // Reading the input file
    let root: PathBuf = current_dir().expect("Not a valid directory.");
    let reader: BufReader<File> = get_reader(&root.join("src").join("day_6").join("input.txt"));

    for line in reader.lines() {
        let line = line.expect("Not able to read line");
    }

    Ok(())
}
