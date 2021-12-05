use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub fn challengefive() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge5/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    //    let output = part_one(input)?;  println!("The output of the first part for challenge 5 is {}", output);
    let output = part_two(input)?;
    println!(
        "The output of the second part for challenge 5 is {}",
        output
    );
    Ok(())
}

type Values = Vec<(i32, i32, i32, i32)>;

fn part_one(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let formatted_input = format_input(input);
    let only_horizontal_vertical = formatted_input
        .into_iter()
        .filter(|&(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .collect::<Values>();
    Ok(calculate_board(only_horizontal_vertical))
}

fn calculate_board(input: Values) -> usize {
    let mut points = HashMap::new();
    for (x1, y1, x2, y2) in input {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        let (mut x, mut y) = (x1, y1);
        while (x, y) != (x2 + dx, y2 + dy) {
            *points.entry((x, y)).or_insert(0) += 1;
            x += dx;
            y += dy;
        }
    }
    points.values().filter(|&&n| n > 1).count()
}

fn format_input(input: Vec<String>) -> Vec<(i32, i32, i32, i32)> {
    let output = input
        .into_iter()
        .filter_map(|l| {
            l.split(" -> ")
                .map(|s| s.split(","))
                .flatten()
                .map(|i| i.parse().unwrap())
                .collect_tuple()
        })
        .collect::<Vec<_>>();
    output
}

fn part_two(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let formatted_input = format_input(input);
    Ok(calculate_board(formatted_input))
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;
    #[test]
    fn part_one_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge5/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>();
        let output = part_one(input)?;
        assert_eq!(output, 5);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge5/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>();
        let output = part_two(input)?;
        assert_eq!(output, 12);
        Ok(())
    }
}
