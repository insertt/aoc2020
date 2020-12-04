use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/day01/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input);
    println!("<----------- INPUT ----------->");

    let numbers = input
        .lines()
        .map(|line| line.parse::<isize>().expect("input line is not a number"))
        .collect::<HashSet<_>>();

    for number in &numbers {
        let needle = 2020 - number;

        if numbers.contains(&needle) {
            println!(
                "Found two numbers that sum to 2020: {} and {}",
                number, needle
            );
            println!(
                "If you'd multiply them together, you'd get: {}",
                number * needle
            );
            break;
        }
    }

    Ok(())
}
