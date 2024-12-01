use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Zip;
use std::path::PathBuf;
use std::slice::Iter;
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

    // Define output variables
    let mut lh:Vec<i32> = vec![];
    let mut rh:Vec<i32> = vec![];
    let mut diff:Vec<i32> = vec![];
    let mut similarity: i32 = 0;

    // Parse the input
    for line in reader.lines() {
        let owned_line: String = match line {
            Ok(line) => line,
            Err(error) => {
                panic!("Not able to read line: {}", error)
            }
        };

        let temp_parts:Vec<&str> = owned_line.split(" ").collect();

        // Add values to output vectors
        lh.push(temp_parts.first().expect("Did not find a string.").parse().expect("Could not parse the string."));
        rh.push(temp_parts.last().expect("Did not find a string.").parse().expect("Could not parse the string."));

    }

    // Sort both vectors
    lh.sort();
    rh.sort();

    // Q1 - Zipping both vectors
    let it: Zip<Iter<'_, i32>, Iter<'_, i32>> = lh.iter().zip(rh.iter());

    for (lh_value, rh_value) in it {
        let temp_diff = (lh_value - rh_value).abs();
        diff.push(temp_diff);
    }

    let result_q1: i32 = diff.iter().sum();
    println!("Result for Q1 is {}", result_q1);

    // Q2
    for number in lh {
        let temp_occurence: Result<i32, std::num::TryFromIntError> = i32::try_from(rh.iter().filter(|n| **n == number).count());
        // Explicit error handling because `TryFromIntError` does not support `?`
        let right_type_occurence = match temp_occurence {
            Ok(temp_occurence) => temp_occurence,
            Err(err) => {
                panic!("Not able to cast to int {}", err)
            }
        };

        similarity += number * right_type_occurence;
    }

    println!("Result for Q2 is {}", similarity);


    // Necessary to be able to use the `?`-magic
    Ok(())

}
