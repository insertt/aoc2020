use std::{collections::HashMap, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input_data = fs::read_to_string("input/day07/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input_data);
    println!("<----------- INPUT ----------->");

    let target_bag = String::from("shiny gold");
    let mut bags = HashMap::new();

    for line in input_data.lines() {
        let mut split = line.split("contain");
        let bag = split.next().unwrap().replace("bags", "");
        let bag = bag.trim();
        let other = split.next().unwrap().trim();

        let nested_bags = if other.starts_with("no") {
            Vec::new()
        } else {
            other
                .split(',')
                .map(|e| e.trim())
                .map(|e| parse_nested_bag(e))
                .collect::<Vec<(String, usize)>>()
        };

        let bag_name = bag.trim();
        bags.insert(bag_name.to_string(), nested_bags);
    }

    // NOTE: We're subtracting one here, because we don't want to count the root.
    let nested_bag_count = count_nested_bags(&bags, &target_bag) - 1;

    println!(
        "Amount of bags inside bag {}: {}",
        target_bag, nested_bag_count
    );

    Ok(())
}

fn count_nested_bags(bags: &HashMap<String, Vec<(String, usize)>>, root_bag: &String) -> usize {
    let root_bag = bags.get(root_bag).unwrap();

    root_bag
        .iter()
        .map(|(bag, amount)| amount * count_nested_bags(bags, bag))
        .sum::<usize>()
        + 1
}

fn parse_nested_bag(bag: &str) -> (String, usize) {
    let split = bag.split_ascii_whitespace().collect::<Vec<&str>>();
    (
        split[1..split.len() - 1].join(" "),
        split[0].parse::<usize>().unwrap(),
    )
}
