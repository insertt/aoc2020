use std::{collections::HashSet, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input_data = fs::read_to_string("input/day06/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input_data);
    println!("<----------- INPUT ----------->");

    let unique_answer_count = input_data.split("\n\n")
        .map(|group| group.lines().collect::<String>())
        .fold(0, |mut acc, group| {
            let uniq_chars = group.chars().collect::<HashSet<_>>();

            acc += uniq_chars.len();
            acc
        });

    println!("Sum of unique answer count: {}", unique_answer_count);

    Ok(())
}
