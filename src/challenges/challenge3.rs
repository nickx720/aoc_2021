use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

pub fn challengethree() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge3/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let outputone = part_one(input)?;
    println!("The output for first part is {}", outputone);
    Ok(())
}

fn part_one(input: Vec<String>) -> Result<u32, ParseIntError> {
    let gamma = input
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .fold(BTreeMap::new(), |mut acc, cur| {
            for item in 0..cur.len() {
                let counter = acc.entry(item).or_insert((0, 0));
                if cur[item] == '1' {
                    *counter = (counter.0 + 1, counter.1)
                } else {
                    *counter = (counter.0, counter.1 + 1);
                }
            }
            acc
        })
        .iter()
        .map(|(_, &(one, zero))| {
            if one > zero {
                return 1;
            } else {
                return 0;
            };
        })
        .collect::<Vec<i32>>();
    let epsilon: Vec<i32> = gamma
        .iter()
        .map(|&x| if x == 0 { return 1 } else { return 0 })
        .collect();
    let (gamma, epsilon) = (
        convert_to_decimal_from_vec(gamma)?,
        convert_to_decimal_from_vec(epsilon)?,
    );

    Ok(gamma * epsilon)
}

fn convert_to_decimal_from_vec(input: Vec<i32>) -> Result<u32, ParseIntError> {
    let binary_string = input
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    let int_value = u32::from_str_radix(&binary_string, 2)?;
    Ok(int_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_works() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        let output = part_one(input).unwrap();
        assert_eq!(output, 198);
    }
}
