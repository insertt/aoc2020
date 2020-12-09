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

    let mut target_number = 0;
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
            target_number = *number;
            break;
        }
    }

    for i in 0..numbers.len() {
        let mut sum = 0;

        for j in i + 1..numbers.len() {
            sum += numbers[j];

            if sum == target_number {
                let subarray = &numbers[i + 1..=j];
                let min = subarray.iter().min().unwrap();
                let max = subarray.iter().max().unwrap();

                println!("Found subarray that sums to a {}! If you'd sum smallest and largest number in this subarray, you'd get: {}", target_number, min + max);
                return Ok(());
            }
        }
    }

    Ok(())
}
