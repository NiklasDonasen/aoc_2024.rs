use crate::get_reader;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use std::collections::HashMap;

pub fn solve_day_5() -> anyhow::Result<()> {
    // Reading the input file
    let root: PathBuf = current_dir().expect("Not a valid directory.");
    let reader: BufReader<File> = get_reader(&root.join("src").join("day_5").join("input.txt"));

    // Structure the output
    let mut q1: i32 = 0;
    let mut q2: i32 = 0;
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut incorrect_updates: Vec<Vec<i32>> = vec![];

    // Parse the input
    for line in reader.lines() {
        let line = line.expect("Not able to read line");

        if line.contains("|") {
            // Parse order
            let parts: Vec<&str> = line.split("|").collect();
            ordering_rules
                .entry(parts[0].parse().expect("Could not parse."))
                .and_modify(|t| t.push(parts[1].parse().expect("Could not parse.")))
                .or_insert(vec![parts[1].parse().expect("Could not parse.")]);

        } else if line.contains(",") {
            // Check each update - at this point you have already parsed all ordering rules
            let parts: Vec<i32> = line
                .split(",")
                .map(|page| page.parse().expect("Could not parse."))
                .collect();
            
            let mut collected_parts: Vec<i32> = vec![];
            let mut ordered: bool = true;
            for (index, page) in parts.iter().enumerate() {
                if index == 0 {
                    // No need to check - everything comes afterwards
                    collected_parts.push(*page);
                }
                else {
                    collected_parts.push(*page);

                    // You have to check if the page is stored as a key in the HashMap
                    let subset_rules:&Vec<i32> = match ordering_rules.get(page) {
                        Some(_) => ordering_rules.get(page).expect("Did not find the key after all"),
                        None => &vec![]
                    };

                    if subset_rules.iter().any(|item| collected_parts.contains(item)) {
                        ordered = false;
                        break;
                    }
                }
            }
            if ordered {
                let middle_index: usize = (parts.len() as f32 / 2.0).ceil() as usize - 1;
                let middle_page = parts[middle_index];
                q1 += middle_page;
            }
            else {
                incorrect_updates.push(parts);
            }
        };
    }

    // Solve Q2
    for update in incorrect_updates {
        let mut pages_with_counter: Vec<(i32, i32)> = vec![];
        for page in &update {
            // You have to check if the page is stored as a key in the HashMap
            let subset_rules:&Vec<i32> = match ordering_rules.get(page) {
                Some(_) => ordering_rules.get(page).expect("Did not find the key after all"),
                None => &vec![]
            };

            // Idea: you count the intersection between subset rules and the update
            // The element with the highest count is the first element in the ordered update
            let counter = subset_rules.into_iter().filter(|e| update.contains(e)).count() as i32;
            pages_with_counter.push((counter, *page));
        }

        // Now we sort the sorted_pages and then we have it
        pages_with_counter.sort();
        let mut sorted_pages: Vec<i32> = vec![];

        for (_, page) in pages_with_counter {
            sorted_pages.insert(0, page);
        }

        let middle_index: usize = (sorted_pages.len() as f32 / 2.0).ceil() as usize - 1;
        let middle_page = sorted_pages[middle_index];
        q2 += middle_page;
    }

    println!("Answer for Q1 is {q1}");
    println!("Answer for Q2 is {q2}");

    Ok(())
}