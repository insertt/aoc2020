use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input_data = fs::read_to_string("input/day06/input.txt")?;

    println!("<----------- INPUT ----------->");
    println!("{}", input_data);
    println!("<----------- INPUT ----------->");

    let common_answer_sum = input_data.split("\n\n")
        .map(|group| group.lines().collect::<Vec<_>>())
        .fold(0, |mut acc, item| {
            acc += item[0].chars()
                .filter(|c| item.iter().skip(1).all(|e| e.contains(*c)))
                .count();
            acc
        });

    println!("Sum of common answer count: {}", common_answer_sum);

    Ok(())
}
