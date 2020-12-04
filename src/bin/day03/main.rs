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

    let mut x = 0usize;
    let mut tree_count = 0usize;

    for line in input {
        let maybe_tree = line.chars().nth(x % line.len()).unwrap();
        x += 3;

        if maybe_tree == TREE_MARKER {
            tree_count += 1;
        }
    }

    println!("Tree count: {}", tree_count);

    Ok(())
}
