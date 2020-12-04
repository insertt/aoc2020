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

    let key_predicates: Vec<(&str, Box<dyn PassportKeyPredicate>)> = vec![
        ("byr", Box::new(ByrKeyPredicate)),
        ("iyr", Box::new(IyrKeyPredicate)),
        ("eyr", Box::new(EyrKeyPredicate)),
        ("hgt", Box::new(HgtKeyPredicate)),
        ("hcl", Box::new(HclKeyPredicate)),
        ("ecl", Box::new(EclKeyPredicate)),
        ("pid", Box::new(PidKeyPredicate)),
    ];

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
            .map(|element| {
                let mut split = element.split(':');
                (split.next().unwrap(), split.next().unwrap())
            })
            .collect::<Vec<(&str, &str)>>();

        let found = key_predicates.iter().all(|(req_key, predicate)| {
            split
                .iter()
                .any(|(key, value)| key == req_key && predicate.test(value))
        });

        if found {
            println!("Found valid passport: {:?}", passport_data);
            valid_passports += 1;
        }
    }

    println!("Valid passport count: {}", valid_passports);
    Ok(())
}

trait PassportKeyPredicate {
    fn test(&self, input: &str) -> bool;
}

struct ByrKeyPredicate;
impl PassportKeyPredicate for ByrKeyPredicate {
    fn test(&self, input: &str) -> bool {
        let number = match input.parse::<i32>() {
            Ok(number) => number,
            Err(_) => return false,
        };

        (1920..=2002).contains(&number)
    }
}

struct IyrKeyPredicate;
impl PassportKeyPredicate for IyrKeyPredicate {
    fn test(&self, input: &str) -> bool {
        let number = match input.parse::<i32>() {
            Ok(number) => number,
            Err(_) => return false,
        };

        (2010..=2020).contains(&number)
    }
}

struct EyrKeyPredicate;
impl PassportKeyPredicate for EyrKeyPredicate {
    fn test(&self, input: &str) -> bool {
        let number = match input.parse::<i32>() {
            Ok(number) => number,
            Err(_) => return false,
        };

        (2020..=2030).contains(&number)
    }
}

struct HgtKeyPredicate;
impl PassportKeyPredicate for HgtKeyPredicate {
    fn test(&self, input: &str) -> bool {
        match (input.strip_suffix("cm"), input.strip_suffix("in")) {
            (Some(cm_height), None) => match cm_height.parse::<i32>() {
                Ok(number) => (150..=193).contains(&number),
                Err(_) => false,
            },
            (None, Some(in_height)) => match in_height.parse::<i32>() {
                Ok(number) => (59..=76).contains(&number),
                Err(_) => false,
            },
            _ => false,
        }
    }
}

struct HclKeyPredicate;
impl PassportKeyPredicate for HclKeyPredicate {
    fn test(&self, input: &str) -> bool {
        const COLOR_LEN: usize = 6;

        match input.strip_prefix('#') {
            Some(color) => color.len() == COLOR_LEN && color.chars().all(|c| c.is_ascii_hexdigit()),
            None => false,
        }
    }
}

struct EclKeyPredicate;
impl PassportKeyPredicate for EclKeyPredicate {
    fn test(&self, input: &str) -> bool {
        matches!(input, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }
}

struct PidKeyPredicate;
impl PassportKeyPredicate for PidKeyPredicate {
    fn test(&self, input: &str) -> bool {
        const PID_LEN: usize = 9;

        input.len() == PID_LEN && input.chars().all(|c| c.is_ascii_digit())
    }
}
