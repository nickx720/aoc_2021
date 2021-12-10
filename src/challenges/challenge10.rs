use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};

use itertools::Itertools;

pub fn challengeten() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge10/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|x| match x {
            Ok(val) => val,
            Err(_e) => "".to_string(),
        })
        .collect::<Vec<String>>();
    //let output = part_one(input)?;
    // println!("The output of the first part is {}", output);
    let output = part_two(input)?;
    println!("The output of the second part is {}", output);
    Ok(())
}

fn part_one(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let input = input
        .into_iter()
        .map(|item| item.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut score = 0;
    for each_item in input {
        let mut queue: VecDeque<char> = VecDeque::new();
        for item in each_item {
            match item {
                '{' | '<' | '[' | '(' => queue.push_back(item),
                ')' => {
                    if let Some(last_item) = queue.pop_back() {
                        if last_item != '(' {
                            score += 3;
                            break;
                        }
                    }
                }
                '>' => {
                    if let Some(last_item) = queue.pop_back() {
                        if last_item != '<' {
                            score += 25137;
                            break;
                        }
                    }
                }
                ']' => {
                    if let Some(last_item) = queue.pop_back() {
                        if last_item != '[' {
                            score += 57;
                            break;
                        }
                    }
                }
                '}' => {
                    if let Some(last_item) = queue.pop_back() {
                        if last_item != '{' {
                            score += 1197;
                            break;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(score)
}
fn handle_char(stack: &mut Vec<char>, c: char) -> Option<usize> {
    match c {
        '(' | '{' | '[' | '<' => stack.push(c),
        _ => match (stack.pop(), c) {
            (Some('('), ')') => {}
            (Some('['), ']') => {}
            (Some('{'), '}') => {}
            (Some('<'), '>') => {}
            (_, ')') => return Some(3),
            (_, ']') => return Some(57),
            (_, '}') => return Some(1197),
            (_, '>') => return Some(25137),
            _ => unreachable!(),
        },
    }
    None
}
fn complete(s: &str) -> Option<usize> {
    let mut stack = Vec::new();
    if s.chars()
        .map(|c| handle_char(&mut stack, c))
        .any(|o| o.is_some())
    {
        return None;
    }
    let score = stack.iter().rev().fold(0, |score, c| match c {
        '(' => score * 5 + 1,
        '[' => score * 5 + 2,
        '{' => score * 5 + 3,
        '<' => score * 5 + 4,
        _ => unreachable!(),
    });
    Some(score)
}
fn part_two(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let input = input
        .iter()
        .filter_map(|item| complete(item))
        .sorted()
        .collect::<Vec<_>>();
    Ok(input[input.len() / 2])
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
        let file = File::open("assets/challenge10/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| match x {
                Ok(val) => val,
                Err(_e) => "".to_string(),
            })
            .collect::<Vec<String>>();
        let output = part_one(input)?;
        assert_eq!(output, 26397);
        Ok(())
    }

    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge10/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| match x {
                Ok(val) => val,
                Err(_e) => "".to_string(),
            })
            .collect::<Vec<String>>();
        let output = part_two(input)?;
        assert_eq!(output, 288957);
        Ok(())
    }
}
