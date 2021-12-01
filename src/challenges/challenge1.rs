use std::fs::File;
use std::io::{BufRead, BufReader};

use itermore::IterMore;

pub fn challenge_one_partone() -> Result<(), std::io::Error> {
    let file = File::open("assets/challenge1/input1.txt")?;
    let reader = BufReader::new(file);
    // Reading the file line by line
    let (_, count) = reader
        .lines()
        .into_iter()
        .fold((0, -1), |(mut acc, mut count), x| {
            let value = x.unwrap().parse::<i32>().unwrap();
            if value > acc {
                acc = value;
                count += 1;
                (acc, count)
            } else {
                (value, count)
            }
        });
    println!("{}", count);
    Ok(())
}

pub fn challenge_one_parttwo() -> Result<(), std::io::Error> {
    let file = File::open("assets/challenge1/input1.txt")?;
    let reader = BufReader::new(file);
    // Reading the file line by line
    let output = reader
        .lines()
        .into_iter()
        .map(|x| x.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let count = output.iter().windows().filter(|[a, _, _, b]| b > a).count();
    println!("{}", count);
    Ok(())
}
