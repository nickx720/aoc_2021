use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn challengetwo() -> Result<(), std::io::Error> {
    let file = File::open("assets/challenge2/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let output = part_one(input);
    println!("The first output is {}", output);
    Ok(())
}

fn part_one(input: Vec<String>) -> u32 {
    let count = input
        .iter()
        .map(|x| {
            let value = x.split(" ").collect::<Vec<&str>>();
            (value[0], value[1])
        })
        .fold((0u32, 0u32), |acc, x| match x {
            ("forward", val) => {
                let value = val.parse::<u32>().unwrap();
                let new_acc = acc.0 + value;
                (new_acc, acc.1)
            }
            ("down", val) => {
                let value = val.parse::<u32>().unwrap();
                let new_acc = acc.1 + value;
                (acc.0, new_acc)
            }
            ("up", val) => {
                let value = val.parse::<u32>().unwrap();
                let new_acc = acc.1 - value;
                (acc.0, new_acc)
            }
            _ => unreachable!(),
        });

    count.0 * count.1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partone_works() {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let result = part_one(input);
        assert_eq!(150, result)
    }
}
