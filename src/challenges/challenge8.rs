use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengeeight() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge8/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|value| {
            if let Ok(res) = value {
                res
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<String>>();
    //let output = part_one(input)?;
    //println!("The output of the first part is {}", output);
    let output = part_two(input)?;
    println!("The output of the second part is {}", output);
    Ok(())
}

fn part_one(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let output = input
        .iter()
        .map(|x| x.trim().split("|").collect::<Vec<&str>>()[1])
        .map(|x| x.trim().split(" ").collect::<Vec<&str>>())
        .flatten()
        .filter(|item| item.len() == 2 || item.len() == 3 || item.len() == 4 || item.len() == 7)
        .collect::<Vec<&str>>();
    Ok(output.len())
}

fn part_two(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut sum = 0;
    let input = input
        .iter()
        .map(|item| {
            let s = item.split("|").map(|s| s.trim()).collect::<Vec<&str>>();
            (s[0], s[1])
        })
        .collect::<Vec<(&str, &str)>>();
    for (segs, value) in input {
        sum += Decoder::new(segs).decode(value);
    }
    Ok(sum)
}

struct Decoder {
    one: Vec<char>,
    four: Vec<char>,
}

impl Decoder {
    fn new(mixed_wires: &str) -> Decoder {
        let segements = mixed_wires
            .split_whitespace()
            .map(|s| s.trim())
            .collect::<Vec<_>>();
        let one = **segements
            .iter()
            .filter(|s| s.len() == 2)
            .collect::<Vec<_>>()
            .first()
            .unwrap();
        let four = **segements
            .iter()
            .filter(|s| s.len() == 4)
            .collect::<Vec<_>>()
            .first()
            .unwrap();

        Decoder {
            one: one.chars().collect(),
            four: four.chars().collect(),
        }
    }

    fn common_with_one(&self, s: &str) -> usize {
        self.one
            .iter()
            .fold(0, |acc, &c| if s.contains(c) { acc + 1 } else { acc })
    }

    fn common_with_four(&self, s: &str) -> usize {
        self.four
            .iter()
            .fold(0, |acc, &c| if s.contains(c) { acc + 1 } else { acc })
    }

    fn decode_segement(&self, s: &str) -> usize {
        match (s.len(), self.common_with_one(s), self.common_with_four(s)) {
            (2, 2, 2) => 1,
            (5, 1, 2) => 2,
            (5, 2, 3) => 3,
            (4, 2, 4) => 4,
            (5, 1, 3) => 5,
            (6, 1, 3) => 6,
            (3, 2, 2) => 7,
            (7, 2, 4) => 8,
            (6, 2, 4) => 9,
            (6, 2, 3) => 0,
            (_, _, _) => panic!(),
        }
    }

    fn decode(&self, s: &str) -> usize {
        let digits = s
            .trim()
            .split_whitespace()
            .map(|v| v.trim())
            .collect::<Vec<_>>();
        let thousands = self.decode_segement(digits[0]) * 1000;
        let hundreds = self.decode_segement(digits[1]) * 100;
        let tens = self.decode_segement(digits[2]) * 10;
        let ones = self.decode_segement(digits[3]);

        thousands + hundreds + tens + ones
    }
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
        let file = File::open("assets/challenge8/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|value| {
                if let Ok(res) = value {
                    res
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<String>>();
        let output = part_one(input)?;
        assert_eq!(output, 26);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge8/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|value| {
                if let Ok(res) = value {
                    res
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<String>>();
        let output = part_two(input)?;
        assert_eq!(output, 61229);
        Ok(())
    }
}
