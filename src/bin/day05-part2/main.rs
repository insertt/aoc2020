use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input_data = fs::read_to_string("input/day05/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input_data);
    println!("<----------- INPUT ----------->");

    let input = input_data.lines().collect::<Vec<_>>();

    let seats = {
        let mut seat_ids = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'F' | 'L' => '0',
                        'B' | 'R' => '1',
                        c => panic!("unexpected char: {}", c),
                    })
                    .collect::<String>()
            })
            .map(|bin| i32::from_str_radix(&bin, 2).unwrap())
            .collect::<Vec<_>>();

        seat_ids.sort_unstable();
        seat_ids
    };

    for (index, current) in seats.iter().enumerate() {
        let next = seats[index + 1];

        if next - current > 1 {
            println!("ID of your seat: {}", current + 1);
            break;
        }
    }
    Ok(())
}
