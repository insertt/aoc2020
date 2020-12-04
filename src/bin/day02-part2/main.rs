use scan_fmt::scan_fmt;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Copy, Clone)]
struct PasswordRequirements {
    first_pos: usize,
    second_pos: usize,
    needle: char,
}

fn parse_line(line: &str) -> Result<(PasswordRequirements, String)> {
    let (min, max, needle, input) = scan_fmt!(line, "{}-{} {}: {}", usize, usize, char, String)?;

    Ok((
        PasswordRequirements {
            first_pos: min,
            second_pos: max,
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
        let chars = input.chars().collect::<Vec<_>>();
        let first = chars.get(requirements.first_pos - 1);
        let second = chars.get(requirements.second_pos - 1);

        if let (Some(first), Some(second)) = (first, second) {
            match (
                *first == requirements.needle,
                *second == requirements.needle,
            ) {
                (true, false) | (false, true) => {
                    println!(
                        "Found password matching criteria ( {:?} ): {}",
                        requirements, input
                    );
                    valid_pwd_count += 1;
                }
                _ => continue,
            }
        }
    }

    println!("Number of passwords matching criteria: {}", valid_pwd_count);

    Ok(())
}
