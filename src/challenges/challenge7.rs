use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengeseven() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge7/input.txt")?;
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
    //let output = part_one(input)?;
    //println!("The output for day 7 first part is {}", output);
    let output = part_two(input)?;
    println!("The output for day 7 second part is {}", output);
    Ok(())
}

fn part_one(input: String) -> Result<i32, Box<dyn std::error::Error>> {
    let formatted_input = format_input(input);
    let min = *formatted_input.iter().min().unwrap();
    let max = *formatted_input.iter().max().unwrap();
    let output = (min..=max)
        .map(|item| {
            formatted_input
                .iter()
                .map(|val| (val - item).abs())
                .sum::<i32>()
        })
        .min()
        .unwrap();
    Ok(output)
}
fn calculate_diff(n: i32) -> i32 {
    n * (n + 1) / 2
}
fn part_two(input: String) -> Result<i32, Box<dyn std::error::Error>> {
    let formatted_input = format_input(input);
    let min = *formatted_input.iter().min().unwrap();
    let max = *formatted_input.iter().max().unwrap();
    let output = (min..=max)
        .map(|item| {
            formatted_input
                .iter()
                .map(|val| calculate_diff((val - item).abs()))
                .sum::<i32>()
        })
        .min()
        .unwrap();
    Ok(output)
}

fn format_input(input: String) -> Vec<i32> {
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
#[test]
fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
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
    let output = part_two(input)?;
    assert_eq!(output, 168);
    Ok(())
}
