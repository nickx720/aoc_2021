use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengethirteen() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge13/input.txt")?;
    let reader = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();
    let lines: Vec<(u32, u32)> = reader
        .iter()
        .take_while(|x| !x.is_empty())
        .map(|x| x.split(',').collect::<Vec<_>>())
        .map(|x| (x[0].parse::<u32>().unwrap(), x[1].parse::<u32>().unwrap()))
        .collect();
    let folds: Vec<(char, u32)> = reader
        .iter()
        .skip_while(|x| !x.starts_with("fold along "))
        .map(|x| x.strip_prefix("fold along ").unwrap())
        .map(|x| x.split('=').collect::<Vec<_>>())
        .map(|x| (x[0].chars().next().unwrap(), x[1].parse::<u32>().unwrap()))
        .collect();
    //let output = part_one(lines, folds);
    //println!("The output of first part is {}", output);
    let output = part_two(lines, folds);
    println!("{}", output);
    Ok(())
}
fn part_one(input: Vec<(u32, u32)>, folds: Vec<(char, u32)>) -> usize {
    fold(input, &[folds[0]]).len()
}
fn fold(input: Vec<(u32, u32)>, folds: &[(char, u32)]) -> Vec<(u32, u32)> {
    let mut map = input.to_vec();
    for fold in folds {
        match *fold {
            ('x', x) => {
                let dots = map.iter().filter(|i| i.0 > x).copied().collect::<Vec<_>>();
                map = map.into_iter().filter(|i| i.0 < x).collect();
                for dot in dots.iter() {
                    map.push((x - (dot.0 - x), dot.1))
                }
            }
            ('y', y) => {
                let dots = map.iter().filter(|i| i.1 > y).copied().collect::<Vec<_>>();
                map = map.into_iter().filter(|i| i.1 < y).collect();
                for dot in dots.iter() {
                    map.push((dot.0, y - (dot.1 - y)))
                }
            }
            _ => unreachable!(),
        }
    }
    map.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn part_two(input: Vec<(u32, u32)>, folds: Vec<(char, u32)>) -> String {
    let pos = fold(input, &folds);
    let mut s = String::new();
    for y in 0..=pos.iter().max_by(|l, r| l.1.cmp(&r.1)).unwrap().1 {
        for x in 0..=pos.iter().max_by(|l, r| l.0.cmp(&r.0)).unwrap().0 {
            if pos.contains(&(x, y)) {
                s.push('#')
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    s
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
        let file = File::open("assets/challenge13/sample.txt")?;
        let reader = BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<String>>();
        let lines: Vec<(u32, u32)> = reader
            .iter()
            .take_while(|x| !x.is_empty())
            .map(|x| x.split(',').collect::<Vec<_>>())
            .map(|x| (x[0].parse::<u32>().unwrap(), x[1].parse::<u32>().unwrap()))
            .collect();
        let folds: Vec<(char, u32)> = reader
            .iter()
            .skip_while(|x| !x.starts_with("fold along "))
            .map(|x| x.strip_prefix("fold along ").unwrap())
            .map(|x| x.split('=').collect::<Vec<_>>())
            .map(|x| (x[0].chars().next().unwrap(), x[1].parse::<u32>().unwrap()))
            .collect();
        let output = part_one(lines, folds);
        assert_eq!(output, 17);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge13/sample.txt")?;
        let reader = BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<String>>();
        let lines: Vec<(u32, u32)> = reader
            .iter()
            .take_while(|x| !x.is_empty())
            .map(|x| x.split(',').collect::<Vec<_>>())
            .map(|x| (x[0].parse::<u32>().unwrap(), x[1].parse::<u32>().unwrap()))
            .collect();
        let folds: Vec<(char, u32)> = reader
            .iter()
            .skip_while(|x| !x.starts_with("fold along "))
            .map(|x| x.strip_prefix("fold along ").unwrap())
            .map(|x| x.split('=').collect::<Vec<_>>())
            .map(|x| (x[0].chars().next().unwrap(), x[1].parse::<u32>().unwrap()))
            .collect();
        let output = part_two(lines, folds);
        assert!(output.len() > 0);
        Ok(())
    }
}
