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
    //   let output = part_one(input);
    // println!("The output of challenge six first part is {}", output);
    println!("Starting");
    let output = part_two(input);
    println!("The output of challenge six second part is {}", output);
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
type Fish = [u128; 9];
fn step_day(val: Fish) -> Fish {
    [
        val[1],
        val[2],
        val[3],
        val[4],
        val[5],
        val[6],
        val[7] + val[0],
        val[8],
        val[0],
    ]
}
fn step_days(days: usize, fish: Fish) -> Fish {
    (0..days).fold(fish, |acc, _| step_day(acc))
}
fn part_two(input: String) -> u128 {
    let formatted_input =
        format_input(input)
            .into_iter()
            .map(|x| x as usize)
            .fold([0u128; 9], |mut acc, f| {
                acc[f] += 1;
                acc
            });

    step_days(256, formatted_input).iter().sum()
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
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge6/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter(|x| x.is_ok())
            .map(|x| if let Ok(val) = x { val } else { unreachable!() })
            .collect::<String>();
        let output = part_two(input);
        assert_eq!(output, 26984457539);
        Ok(())
    }
}
