use crate::get_reader;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn solve_day_4() -> anyhow::Result<()> {
     // Reading the input file
     let root: PathBuf = current_dir().expect("Not a valid directory.");
     let reader: BufReader<File> = get_reader(&root.join("src").join("day_4").join("input.txt"));
 
     // You only need to store relevant letters
     let mut x_loc: Vec<(i32, i32)> = vec![];
     let mut m_loc: Vec<(i32, i32)> = vec![];
     let mut a_loc: Vec<(i32, i32)> = vec![];
     let mut s_loc: Vec<(i32, i32)> = vec![];
 
     // Store the result
     let mut q1_occurences: i32 = 0;
     let mut q2_occurences: i32 = 0;
 
     // Parse the input
     let mut row: i32 = 0;
     for line in reader.lines() {
         let line = line.expect("Not able to read line");
 
         for (index, elem) in line.char_indices() {
             match elem {
                 'X' => x_loc.push((row, index.try_into().unwrap())),
                 'M' => m_loc.push((row, index.try_into().unwrap())),
                 'A' => a_loc.push((row, index.try_into().unwrap())),
                 'S' => s_loc.push((row, index.try_into().unwrap())),
                 _ => (),
             }
         }
         row += 1;
     }
 
     // Q1 Always start from an `X`
     for (row, col) in x_loc {
         // Search in all directions
         let possible_locs: Vec<(i32, i32, &str)> = vec![
             (row, col - 1, "left"),
             (row - 1, col - 1, "leftup"),
             (row - 1, col, "up"),
             (row - 1, col + 1, "rightup"),
             (row, col + 1, "right"),
             (row + 1, col + 1, "rightdown"),
             (row + 1, col, "down"),
             (row + 1, col - 1, "downleft"),
         ];
 
         // Check if one of the values is in m_loc
         for loc in possible_locs {
             if m_loc.contains(&(loc.0, loc.1)) {
                 // You may have an XMAS - check a_loc
                 let next_loc = return_next_loc(&(loc.0, loc.1), loc.2);
                 if a_loc.contains(&(next_loc.0, next_loc.1)) {
                     // Even more exciting - check s_loc
                     let next_loc = return_next_loc(&(next_loc.0, next_loc.1), loc.2);
                     if s_loc.contains(&(next_loc.0, next_loc.1)) {
                         q1_occurences += 1;
                     }
                 }
             }
         }
     }
 
     // Q2 Always start from an `A`
     for (row, col) in a_loc {
         let mut counter = 0;
         // Search in allowed directions
         let possible_locs: Vec<(i32, i32, &str)> = vec![
             (row - 1, col - 1, "leftup"),
             (row - 1, col + 1, "rightup"),
             (row + 1, col + 1, "rightdown"),
             (row + 1, col - 1, "downleft"),
         ];
 
         // Check if one of the values is in m_loc
         for loc in possible_locs {
             if m_loc.contains(&(loc.0, loc.1)) {
                 // Find opposite direction
                 let opposite_direction: &str = match loc.2 {
                     "leftup" => "rightdown",
                     "rightdown" => "leftup",
                     "rightup" => "downleft",
                     "downleft" => "rightup",
                     _ => panic!("Not a valid move."),
                 };
                 // You may have an MAS - check s_loc
                 let next_loc = return_next_loc(&(row, col), opposite_direction);
                 if s_loc.contains(&(next_loc.0, next_loc.1)) {
                     counter += 1;
                     if counter == 2 {
                         q2_occurences += 1;
                     }
                 }
             }
         }
     }
 
     println!("Answer to Q1 is {q1_occurences}.");
     println!("Answer to Q2 is {q2_occurences}.");
 
     Ok(())
}

fn return_next_loc(cur_loc: &(i32, i32), direction: &str) -> (i32, i32) {
    match direction {
        "left" => (cur_loc.0, cur_loc.1 - 1),
        "leftup" => (cur_loc.0 - 1, cur_loc.1 - 1),
        "up" => (cur_loc.0 - 1, cur_loc.1),
        "rightup" => (cur_loc.0 - 1, cur_loc.1 + 1),
        "right" => (cur_loc.0, cur_loc.1 + 1),
        "rightdown" => (cur_loc.0 + 1, cur_loc.1 + 1),
        "down" => (cur_loc.0 + 1, cur_loc.1),
        "downleft" => (cur_loc.0 + 1, cur_loc.1 - 1),
        _ => panic!("Not a valid move"),
    }
}
