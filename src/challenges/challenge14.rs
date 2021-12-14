use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub fn challengefourteen() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge14/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();
    //let output = part_one(input)?;
    //println!("The output of the first part is {}", output);
    let output = part_two(input)?;
    println!("The output of the first part is {}", output);
    Ok(())
}

fn compute_delta(pair_freqs: &HashMap<(char, char), usize>, last_char: char) -> usize {
    let mut char_freqs: HashMap<char, usize> = HashMap::new();
    for (k, v) in pair_freqs {
        *char_freqs.entry(k.0).or_default() += v;
    }

    (*char_freqs.entry(last_char).or_default()) += 1;

    char_freqs.values().max().unwrap() - char_freqs.values().min().unwrap()
}

fn part_one(inputs: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let polymer = inputs[0].chars().collect_vec();
    let last_char = *polymer.last().unwrap();

    let mut rules = HashMap::new();
    for rule in &inputs[2..] {
        let (lhs, rhs) = rule.split_once(" -> ").unwrap();
        let lhs = lhs.chars().collect_vec();
        rules.insert((lhs[0], lhs[1]), rhs.chars().next().unwrap());
    }
    let mut pair_freqs: HashMap<(char, char), usize> = HashMap::new();
    for (a, b) in polymer.iter().tuple_windows() {
        (*pair_freqs.entry((*a, *b)).or_default()) += 1;
    }
    for i in 0..40 {
        let mut new_pair_freqs: HashMap<(char, char), usize> = HashMap::new();
        for (&(a, b), v) in &pair_freqs {
            let to_insert = *rules.get(&(a, b)).unwrap();
            (*new_pair_freqs.entry((a, to_insert)).or_default()) += v;
            (*new_pair_freqs.entry((to_insert, b)).or_default()) += v;
        }
        pair_freqs = new_pair_freqs;
        if i == 9 {
            let output = compute_delta(&pair_freqs, last_char);
            return Ok(output);
        }
    }
    Ok(0)
}

fn part_two(inputs: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let polymer = inputs[0].chars().collect_vec();
    let last_char = *polymer.last().unwrap();

    let mut rules = HashMap::new();
    for rule in &inputs[2..] {
        let (lhs, rhs) = rule.split_once(" -> ").unwrap();
        let lhs = lhs.chars().collect_vec();
        rules.insert((lhs[0], lhs[1]), rhs.chars().next().unwrap());
    }
    let mut pair_freqs: HashMap<(char, char), usize> = HashMap::new();
    for (a, b) in polymer.iter().tuple_windows() {
        (*pair_freqs.entry((*a, *b)).or_default()) += 1;
    }
    for i in 0..40 {
        let mut new_pair_freqs: HashMap<(char, char), usize> = HashMap::new();
        for (&(a, b), v) in &pair_freqs {
            let to_insert = *rules.get(&(a, b)).unwrap();
            (*new_pair_freqs.entry((a, to_insert)).or_default()) += v;
            (*new_pair_freqs.entry((to_insert, b)).or_default()) += v;
        }
        pair_freqs = new_pair_freqs;
    }
    Ok(compute_delta(&pair_freqs, last_char))
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
        let file = File::open("assets/challenge14/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter_map(|l| l.ok())
            .collect::<Vec<String>>();
        let output = part_one(input)?;
        assert_eq!(output, 1588);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge14/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter_map(|l| l.ok())
            .collect::<Vec<String>>();
        let output = part_two(input)?;
        assert_eq!(output, 2188189693529);
        Ok(())
    }
}
