use itertools::Itertools;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input_data = fs::read_to_string("input/day09/input.txt")?;
    const PREAMBLE_LENGTH: usize = 25;

    let numbers = input_data
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for i in 0..numbers.len() {
        let preamble = &numbers[i..i + PREAMBLE_LENGTH];
        let number = &numbers[i + PREAMBLE_LENGTH];

        let found = preamble
            .iter()
            .cartesian_product(preamble)
            .filter(|(a, b)| a != b)
            .any(|(a, b)| a + b == *number);

        if !found {
            println!(
                "Found first number that is not produced from the numbers from the preamble: {}",
                number
            );
            break;
        }
    }

    Ok(())
}
