use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bitvec::prelude::*;
pub fn challengesixteen() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge16/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(|item| item.ok())
        .collect::<String>();
    //let output = part_one(input)?;
    //println!("The output for the first part is {}", output);
    let output = part_two(input)?;
    println!("The output for the first part is {}", output);
    Ok(())
}
struct Bits {
    bits: Vec<u64>,
    pos: usize,
}

impl Bits {
    fn take(&mut self, n: usize) -> &[u64] {
        self.pos += n;
        &self.bits[self.pos - n..self.pos]
    }
}

fn bin(ds: &[u64]) -> u64 {
    ds.iter().fold(0, |a, b| a << 1 | b)
}

fn packet(bs: &mut Bits) -> (u64, u64) {
    let mut version = bin(bs.take(3));
    let type_id = bin(bs.take(3));
    if type_id == 4 {
        let mut n = 0;
        loop {
            let ds = bs.take(5);
            n = n << 4 | bin(&ds[1..]);
            if ds[0] == 0 {
                return (version, n);
            }
        }
    }
    let mut ns = vec![];
    if bs.take(1)[0] == 0 {
        let n = bin(bs.take(15)) as usize + bs.pos;
        while bs.pos < n {
            let (v, a) = packet(bs);
            version += v;
            ns.push(a);
        }
    } else {
        for _ in 0..bin(bs.take(11)) {
            let (v, a) = packet(bs);
            version += v;
            ns.push(a);
        }
    }
    let n = match type_id {
        0 => ns.into_iter().sum(),
        1 => ns.into_iter().product(),
        2 => ns.into_iter().min().unwrap(),
        3 => ns.into_iter().max().unwrap(),
        5 => (ns[0] > ns[1]) as u64,
        6 => (ns[0] < ns[1]) as u64,
        7 => (ns[0] == ns[1]) as u64,
        _ => panic!("Bad type id: {}", type_id),
    };
    (version, n)
}

fn solve(input: &str) -> (u64, u64) {
    packet(&mut Bits {
        bits: input
            .chars()
            .flat_map(|c| {
                let n = c.to_digit(16).unwrap() as u64;
                vec![n >> 3 & 1, n >> 2 & 1, n >> 1 & 1, n & 1]
            })
            .collect(),
        pos: 0,
    })
}
fn part_one(input: String) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(solve(&input).0)
}

fn part_two(input: String) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(solve(&input).1)
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
        let file = File::open("assets/challenge16/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter_map(|item| item.ok())
            .collect::<String>();
        let output = part_one(input)?;
        assert_eq!(output, 16);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let input = String::from("C200B40A82");
        let output = part_two(input)?;
        assert_eq!(output, 3);
        Ok(())
    }
}
