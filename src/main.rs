use std::fs::File;
use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// mod day_1;

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
    let reader: BufReader<File> = get_reader(&root.join("src").join("day_2").join("input.txt"));

    // Structure the output
    let mut safe_reps: i32 = 0;
    let mut safe_reps_w_dampener: i32 = 0;

    // Parse the input
    for line in reader.lines() {
        let line = line.expect("Not able to read line");

        let line_parts: Vec<&str> = line.split(" ").collect();

        let mut report: Vec<i32> = vec![];

        for level in line_parts {
            let level: i32 = level.parse().expect("Could not parse string to int.");
            report.push(level);
        }

        // Q1 - Check if vector fullfils both conditions
        if check_vector(&report) {
            safe_reps += 1;
            safe_reps_w_dampener += 1;
        }
        else {
            // Q2 - apply the problem dampener
            let num_elements = report.len();
            for index in 0..num_elements {
                let mut temp_report = report.clone();
                temp_report.remove(index);

                // Check until you get a true
                if check_vector(&temp_report) {
                    safe_reps_w_dampener += 1;
                    break;
                }
            }
        }
    }

    println!("Answer to Q1 is {safe_reps}.");
    println!("Answer to Q2 is {safe_reps_w_dampener}.");


    Ok(())
}

fn check_vector(input: &Vec<i32>) -> bool {
    // first condition: all in- or decreasing
    let increasing: bool = input.windows(2).all(|w| w[0] <= w[1]);
    let decreasing: bool = input.windows(2).all(|w| w[0] >= w[1]);

    // Check the conditions
    if increasing || decreasing {
        println!("First condition satisfied");
        // Now you can check second condition
        // second condition: no big jumps
        let jumps: bool = input.windows(2).all(|w| 1 <= (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3);
        if jumps {
            return  true
        }
    }
    else {
        return false;
    }

    false
}
