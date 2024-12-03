use crate::get_reader;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use regex::Regex;

pub fn solve_day_3() -> anyhow::Result<()> {
    // Reading the input file
    let root: PathBuf = current_dir().expect("Not a valid directory.");
    let reader: BufReader<File> = get_reader(&root.join("src").join("day_3").join("input.txt"));

    // Structure output
    let mut answer_q1: i32 = 0;
    let mut answer_q2: i32 = 0;

    let mut activated: bool = true;

    // Parse the input
    for line in reader.lines() {
        let line = line.expect("Not able to read line");

        // Regex magic
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

        let it = re.captures_iter(&line);

        for matched_string in it {
            let first_value: i32 = matched_string[1]
                .parse()
                .expect("Could not parse from string.");
            let second_value: i32 = matched_string[2]
                .parse()
                .expect("Could not parse from string.");
            answer_q1 += first_value * second_value;
        }

        // Q2
        // Regex magic
        let re_q2 = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(don't\(\)|do\(\))").unwrap();

        let it = re_q2.captures_iter(&line);

        for matched_string in it {
            if matched_string[0].contains("do()") {
                activated = true;
            } else if matched_string[0].contains("don't()") {
                activated = false;
            } else if activated {
                let first_value: i32 = matched_string[1]
                    .parse()
                    .expect("Could not parse from string.");
                let second_value: i32 = matched_string[2]
                    .parse()
                    .expect("Could not parse from string.");

                answer_q2 += first_value * second_value;
            }
        }
    }

    println!("Answer to Q1 is {answer_q1}");
    println!("Answer to Q2 is {answer_q2}");

    Ok(())
}
