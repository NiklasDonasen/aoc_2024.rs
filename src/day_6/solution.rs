use crate::get_reader;
use std::collections::HashSet;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;


pub fn solve_day_6() -> anyhow::Result<()> {
    // Reading the input file
    let root: PathBuf = current_dir().expect("Not a valid directory.");
    let reader: BufReader<File> = get_reader(&root.join("src").join("day_6").join("input.txt"));

    // Store output
    let mut obstructions: Vec<(i32, i32)> = vec![];
    let mut guard_pos: GuardPos = GuardPos {
        row: 0,
        col: 0,
        direction: "NOT_IMPLEMENTED",
    };

    // Parse the input
    let mut row: i32 = 0;
    let mut max_index: i32 = 0;
    for line in reader.lines() {
        let line = line.expect("Not able to read line");
        max_index = line.len() as i32;

        for (index, element) in line.char_indices() {
            match element {
                '#' => obstructions.push((row, index as i32)),
                '^' => guard_pos = update_guard_pos(guard_pos, row, index as i32, "up"),
                '<' => guard_pos = update_guard_pos(guard_pos, row, index as i32, "left"),
                '>' => guard_pos = update_guard_pos(guard_pos, row, index as i32, "right"),
                '.' => (),
                _ => panic!("Unexpected element"),
            }
        }
        row += 1;
    }
    // Just to be explicit we store it in a separate variable
    let max_row: i32 = row;

    // Q1
    let mut inside = true_if_inside(max_row, max_index, guard_pos.row, guard_pos.col);
    let mut unique_locs: HashSet<(i32, i32)> = HashSet::new();
    // Store the start position for Q2
    let start: (i32, i32) = (guard_pos.row, guard_pos.col);
    let start_direction: &str = guard_pos.direction;

    while inside {
        unique_locs.insert((guard_pos.row, guard_pos.col));
        let mut next_loc = return_next_loc((guard_pos.row, guard_pos.col), guard_pos.direction);
        if obstructions.contains(&(next_loc.0, next_loc.1)) {
            let new_direction: &str = match guard_pos.direction {
                "up" => "right",
                "right" => "down",
                "down" => "left",
                "left" => "up",
                _ => panic!("Not a valid direction"),
            };
            next_loc = return_next_loc((guard_pos.row, guard_pos.col), new_direction);
            if obstructions.contains(&(next_loc.0, next_loc.1)) {
                todo!("Implement several rotations");
            }
        }

        guard_pos = update_guard_pos(guard_pos, next_loc.0, next_loc.1, next_loc.2);

        inside = true_if_inside(max_row, max_index, guard_pos.row, guard_pos.col);
    }
    println!("Answer for Q1 is {}", unique_locs.len());

    // Q2
    // remove the start position because you cannot put an obstacle there
    unique_locs.remove(&start);
    let mut loop_obstacles: i32 = 0;

    for loc in unique_locs {
        // add the obstruction at the very end
        obstructions.push(loc);

        // Initialize for a new round
        let mut visited_locations: HashSet<(i32, i32)> = HashSet::new();
        guard_pos = update_guard_pos(guard_pos, start.0, start.1, start_direction);

        // Variable to see if we are no longer adding new values to visited_locations
        let mut constant_visited_locations: i32 = 0;

        while constant_visited_locations < 500 {
            if visited_locations.contains(&(guard_pos.row, guard_pos.col)) {
                constant_visited_locations += 1;
            } else {
                // As soon as we add a new location, we re-set the constant-counter
                visited_locations.insert((guard_pos.row, guard_pos.col));
                constant_visited_locations = 0;
            }
            let mut next_loc = return_next_loc((guard_pos.row, guard_pos.col), guard_pos.direction);
            if obstructions.contains(&(next_loc.0, next_loc.1)) {
                let new_direction: &str = match guard_pos.direction {
                    "up" => "right",
                    "right" => "down",
                    "down" => "left",
                    "left" => "up",
                    _ => panic!("Not a valid direction"),
                };
                next_loc = return_next_loc((guard_pos.row, guard_pos.col), new_direction);
                if obstructions.contains(&(next_loc.0, next_loc.1)) {
                    let new_direction: &str = match new_direction {
                        "up" => "right",
                        "right" => "down",
                        "down" => "left",
                        "left" => "up",
                        _ => panic!("Not a valid direction"),
                    };
                    next_loc = return_next_loc((guard_pos.row, guard_pos.col), new_direction);
                    if obstructions.contains(&(next_loc.0, next_loc.1)) {
                        todo!("Implement several rotations");
                    }
                }
            }

            guard_pos = update_guard_pos(guard_pos, next_loc.0, next_loc.1, next_loc.2);

            inside = true_if_inside(max_row, max_index, guard_pos.row, guard_pos.col);

            if !inside {
                break;
            }
        }

        if inside {
            loop_obstacles += 1;
            println!("{:?}", loc);
        }

        // Remove the new obstruction again
        obstructions.pop();
    }

    println!("Answer for Q2 is {loop_obstacles}");

    Ok(())
}

struct GuardPos<'life> {
    row: i32,
    col: i32,
    direction: &'life str,
}

fn update_guard_pos<'life>(
    mut guard_pos: GuardPos<'life>,
    row: i32,
    col: i32,
    direction: &'life str,
) -> GuardPos<'life> {
    guard_pos.row = row;
    guard_pos.col = col;
    guard_pos.direction = direction;

    guard_pos
}

fn true_if_inside(max_row: i32, max_col: i32, cur_row: i32, cur_col: i32) -> bool {
    // Check row
    if (0 <= cur_row) && (cur_row <= max_row) {
        // Check col
        if (0 <= cur_col) && (cur_col <= max_col) {
            return true;
        }
    }
    false
}

fn return_next_loc(cur_loc: (i32, i32), direction: &str) -> (i32, i32, &str) {
    match direction {
        "left" => (cur_loc.0, cur_loc.1 - 1, "left"),
        "leftup" => (cur_loc.0 - 1, cur_loc.1 - 1, "leftup"),
        "up" => (cur_loc.0 - 1, cur_loc.1, "up"),
        "rightup" => (cur_loc.0 - 1, cur_loc.1 + 1, "rightup"),
        "right" => (cur_loc.0, cur_loc.1 + 1, "right"),
        "rightdown" => (cur_loc.0 + 1, cur_loc.1 + 1, "rightdown"),
        "down" => (cur_loc.0 + 1, cur_loc.1, "down"),
        "downleft" => (cur_loc.0 + 1, cur_loc.1 - 1, "downleft"),
        _ => panic!("Not a valid move"),
    }
}