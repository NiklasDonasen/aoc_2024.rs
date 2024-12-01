use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod day_1;
use day_1::solution::solve_day_1;

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
    solve_day_1().expect("Did not manage to solve the question.");

    Ok(())
}
