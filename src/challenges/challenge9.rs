use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub fn challengenine() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge9/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|x| if let Ok(val) = x { val } else { "".to_string() })
        .collect::<Vec<String>>();
    //    let output = part_one(input)?;
    //    println!("The output of the first part is {}", output);
    let output = part_two(input)?;
    println!("The output of the second part is {}", output);
    Ok(())
}

fn part_one(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>> {
    let input = input
        .iter()
        .map(|x| {
            x.trim()
                .split("")
                .filter(|&item| item != "".to_string())
                .map(|item| item.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut count = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let current = input[row][col];
            if row != 0 && current >= input[row - 1][col] {
                continue;
            }
            if row + 1 < input.len() && current >= input[row + 1][col] {
                continue;
            }
            if col != 0 && current >= input[row][col - 1] {
                continue;
            }
            if col + 1 < input[row].len() && current >= input[row][col + 1] {
                continue;
            }
            count += current + 1;
        }
    }
    Ok(count)
}

fn remove_component((x, y): (i32, i32), coords: &mut HashSet<(i32, i32)>) -> usize {
    if !coords.remove(&(x, y)) {
        return 0;
    }
    1 + [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .iter()
        .map(|&ne| remove_component(ne, coords))
        .sum::<usize>()
}

fn part_two(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let input = input
        .iter()
        .map(|item| item.bytes().map(|c| c - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let mut points = (0..input[0].len())
        .cartesian_product(0..input.len())
        .filter(|&(x, y)| input[y][x] != 9)
        .map(|(x, y)| (x as i32, y as i32))
        .collect::<HashSet<_>>();
    let mut cs = vec![];
    while let Some(&p) = points.iter().next() {
        cs.push(remove_component(p, &mut points));
    }
    Ok(cs.iter().sorted().rev().take(3).product())
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
        let file = File::open("assets/challenge9/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| if let Ok(val) = x { val } else { "".to_string() })
            .collect::<Vec<String>>();
        let output = part_one(input)?;
        assert_eq!(output, 15);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge9/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| if let Ok(val) = x { val } else { "".to_string() })
            .collect::<Vec<String>>();
        let output = part_two(input)?;
        assert_eq!(output, 1134);
        Ok(())
    }
}
