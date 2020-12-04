use std::{fs, mem};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input_data = fs::read_to_string("input/day04/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input_data);
    println!("<----------- INPUT ----------->");

    let input = input_data
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let key_predicates: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut passport_data: Vec<String> = Vec::new();
    let mut current_state = String::new();

    for (index, line) in input.iter().enumerate() {
        if line.is_empty() {
            let line = mem::take(&mut current_state);
            passport_data.push(line.trim_end().to_string());
            continue;
        }

        current_state += &line.trim_end();

        if index != input.len() - 1 {
            current_state += " ";
        } else {
            passport_data.push(current_state);
            break;
        }
    }

    let mut valid_passports = 0usize;
    for passport_data in passport_data {
        let split = passport_data
            .split(' ')
            .filter(|e| !e.is_empty())
            .map(|element| element.split(':').next().unwrap())
            .collect::<Vec<&str>>();

        let found = key_predicates
            .iter()
            .all(|req_key| split.iter().any(|key| key == req_key));

        if found {
            println!("Found valid passport: {:?}", passport_data);
            valid_passports += 1;
        }
    }

    println!("Valid passport count: {}", valid_passports);
    Ok(())
}
