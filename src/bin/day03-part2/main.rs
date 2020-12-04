#![feature(bindings_after_at)]

use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const TREE_MARKER: char = '#';

fn main() -> Result<()> {
    let input_data = fs::read_to_string("input/day03/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input_data);
    println!("<----------- INPUT ----------->");

    let input = input_data
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut mul_result = 1;
    for slope @ (slope_x, slope_y) in slopes {
        let mut x = 0usize;
        let mut tree_count = 0usize;

        for line in input.iter().step_by(slope_y) {
            let maybe_tree = line.chars().nth(x % line.len()).unwrap();
            x += slope_x;

            if maybe_tree == TREE_MARKER {
                tree_count += 1;
            }
        }

        mul_result *= tree_count;
        println!("For slope {:?} tree count is: {}", slope, tree_count);
    }

    println!(
        "If you'd multiply tree count for every slope, you'd get: {}",
        mul_result
    );
    Ok(())
}
