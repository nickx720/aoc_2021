use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengesix() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge6/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| if let Ok(val) = x { val } else { unreachable!() })
        .collect::<String>();
    let output = part_one(input);
    println!("The output of challenge six first part is {}", output);
    Ok(())
}
fn part_one(input: String) -> usize {
    let mut formatted_input = format_input(input);
    let mut index = 80;
    loop {
        if index == 0 {
            break;
        }
        if formatted_input.iter().any(|&x| x == 0) {
            let count = formatted_input.iter().filter(|x| **x == 0u32).count();
            formatted_input = formatted_input
                .iter()
                .map(|&x| if x == 0 { 7 } else { x })
                .collect();
            for _ in 0..count {
                formatted_input.push(9);
            }
        }
        formatted_input = formatted_input.iter().map(|x| x - 1).collect();

        index -= 1;
    }
    formatted_input.len()
}
fn format_input(input: String) -> Vec<u32> {
    input
        .split(",")
        .map(|item| {
            if let Ok(val) = item.parse::<u32>() {
                val
            } else {
                0
            }
        })
        .collect()
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
        let file = File::open("assets/challenge6/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter(|x| x.is_ok())
            .map(|x| if let Ok(val) = x { val } else { unreachable!() })
            .collect::<String>();
        let output = part_one(input);
        assert_eq!(output, 5934);
        Ok(())
    }
}
