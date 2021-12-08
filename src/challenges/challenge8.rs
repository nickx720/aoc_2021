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
    let output = part_one(input)?;
    println!("The output of the first part is {}", output);
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
}
