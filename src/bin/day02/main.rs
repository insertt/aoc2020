use scan_fmt::scan_fmt;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Copy, Clone)]
struct PasswordRequirements {
    min_length: usize,
    max_length: usize,
    needle: char,
}

fn parse_line(line: &str) -> Result<(PasswordRequirements, String)> {
    let (min, max, needle, input) = scan_fmt!(line, "{}-{} {}: {}", usize, usize, char, String)?;

    Ok((
        PasswordRequirements {
            min_length: min,
            max_length: max,
            needle,
        },
        input,
    ))
}

fn main() -> Result<()> {
    let input_data = fs::read_to_string("input/day02/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input_data);
    println!("<----------- INPUT ----------->");

    let input = input_data
        .lines()
        .map(|line| parse_line(line).unwrap_or_else(|_| panic!("invalid format line: {}", line)))
        .collect::<Vec<_>>();

    let mut valid_pwd_count = 0;
    for (requirements, input) in &input {
        let count = input.chars().filter(|c| *c == requirements.needle).count();

        if count < requirements.min_length || count > requirements.max_length {
            continue;
        }

        println!(
            "Found password matching criteria ( {:?} ): {}",
            requirements, input
        );
        valid_pwd_count += 1;
    }

    println!("Number of passwords matching criteria: {}", valid_pwd_count);

    Ok(())
}
