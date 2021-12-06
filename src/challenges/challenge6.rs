pub fn challengesix() -> Result<(), Box<dyn std::error::Error>> {
    println!("The output of challenge six first part is {}", 1);
    Ok(())
}
fn part_one(input: String) -> usize {
    let formatted_input = format_input(input);
    dbg!(formatted_input);
    2
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
