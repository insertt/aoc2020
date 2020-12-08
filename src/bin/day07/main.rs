use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

    let mut can_carry_bags = HashSet::new();
    find_possible_bag_ancestors(&bags, &target_bag, &mut can_carry_bags);

    println!(
        "Amount of bags that can contain {} bag: {}",
        target_bag,
        can_carry_bags.len()
    );
    Ok(())
}

fn find_possible_bag_ancestors(
    bags: &HashMap<String, Vec<(String, usize)>>,
    target_bag: &String,
    can_carry_bags: &mut HashSet<String>,
) {
    for (bag, nested_bags) in bags {
        if nested_bags.iter().any(|(k, _)| k == target_bag) && can_carry_bags.insert(bag.clone()) {
            find_possible_bag_ancestors(bags, bag, can_carry_bags);
        }
    }
}

fn parse_nested_bag(bag: &str) -> (String, usize) {
    let split = bag.split_ascii_whitespace().collect::<Vec<&str>>();
    (
        split[1..split.len() - 1].join(" "),
        split[0].parse::<usize>().unwrap(),
    )
}
