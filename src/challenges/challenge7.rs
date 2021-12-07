pub fn challengeseven() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello");
    Ok(())
}

fn part_one(input: String) -> Result<usize, Box<dyn std::error::Error>> {
    let formatted_input = format_input(input);
    dbg!(formatted_input);
    Ok(2)
}

fn format_input(input: String) -> Vec<u32> {
    input
        .split(",")
        .map(|num| {
            if let Ok(number) = num.parse() {
                number
            } else {
                0
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    #[test]
    fn part_one_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge7/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| {
                if let Ok(value) = x {
                    value
                } else {
                    "".to_string()
                }
            })
            .collect::<String>();
        let output = part_one(input)?;
        assert_eq!(output, 37);
        Ok(())
    }
}
