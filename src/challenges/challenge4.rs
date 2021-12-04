pub fn challengefour() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello");
    Ok(())
}

fn part_one(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>> {
    let draws = input[0]
        .clone()
        .split(",")
        .map(|x| x.parse::<usize>())
        .filter(|x| x.is_ok())
        .map(|x| {
            let x = x.ok();
            match x {
                Some(val) => val,
                _ => unreachable!(),
            }
        })
        .collect::<Vec<usize>>();
    for i in 5..draws.len() {
        dbg!(i);
    }
    Ok(0)
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;
    #[test]
    fn part_one_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge4/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| x.ok())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>();
        let output = part_one(input)?;
        assert_eq!(output, 4512);
        Ok(())
    }
}
