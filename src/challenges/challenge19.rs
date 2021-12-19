use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use hashbrown::HashSet;
use itertools::Itertools;

pub fn challengenineteen() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge19/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(Result::ok)
        .map(|item| {
            if item == "" {
                "\n\n".to_string()
            } else {
                item + "\n"
            }
        })
        .collect::<String>();
    let input = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .filter(|item| !item.starts_with("---"))
                .map(|l| {
                    let (a, tmp) = l.split_once(',').unwrap();
                    let (b, c) = tmp.split_once(',').unwrap();
                    [a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()]
                })
                .collect::<Vec<[i32; 3]>>()
        })
        .collect::<Vec<_>>();
    //    let output = part_one(input)?;
    //    println!("The output for first part is {}", output);
    let output = part_two(input)?;
    println!("The output for first part is {}", output);
    Ok(())
}

fn rotate([x, y, z]: [i32; 3], rot: u8) -> [i32; 3] {
    match rot {
        0 => [x, y, z],
        1 => [y, -x, z],
        2 => [-x, -y, z],
        3 => [-y, x, z],
        4 => [z, y, -x],
        5 => [y, -z, -x],
        6 => [-z, -y, -x],
        7 => [-y, z, -x],
        8 => [z, -x, -y],
        9 => [-x, -z, -y],
        10 => [-z, x, -y],
        11 => [x, z, -y],
        12 => [z, -y, x],
        13 => [-y, -z, x],
        14 => [-z, y, x],
        15 => [y, z, x],
        16 => [z, x, y],
        17 => [x, -z, y],
        18 => [-z, -x, y],
        19 => [-x, z, y],
        20 => [-x, y, -z],
        21 => [y, x, -z],
        22 => [x, -y, -z],
        23 => [-y, -x, -z],
        _ => unreachable!(),
    }
}

fn merge_scan(total_scan: &mut HashSet<[i32; 3]>, scan: &[[i32; 3]], rot: u8) -> Option<[i32; 3]> {
    let rotated_scan = scan.iter().map(|&v| rotate(v, rot)).collect::<Vec<_>>();
    let distances = total_scan
        .iter()
        .cartesian_product(&rotated_scan)
        .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);
    for [dx, dy, dz] in distances {
        let translated = rotated_scan
            .iter()
            .map(|[x3, y3, z3]| [x3 + dx, y3 + dy, z3 + dz]);
        if translated
            .clone()
            .filter(|v| total_scan.contains(v))
            .count()
            >= 12
        {
            total_scan.extend(translated);
            return Some([dx, dy, dz]);
        }
    }
    None
}

fn part_one(mut scans: Vec<Vec<[i32; 3]>>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut total_scan = scans.swap_remove(0).into_iter().collect::<HashSet<_>>();
    let mut dists = Vec::new();
    while !scans.is_empty() {
        for i in (0..scans.len()).rev() {
            if let Some(d) = (0..24).find_map(|rot| merge_scan(&mut total_scan, &scans[i], rot)) {
                dists.push(d);
                scans.swap_remove(i);
            }
        }
    }
    Ok(total_scan.len())
}

fn part_two(mut scans: Vec<Vec<[i32; 3]>>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total_scan = scans.swap_remove(0).into_iter().collect::<HashSet<_>>();
    let mut dists = Vec::new();
    while !scans.is_empty() {
        for i in (0..scans.len()).rev() {
            if let Some(d) = (0..24).find_map(|rot| merge_scan(&mut total_scan, &scans[i], rot)) {
                dists.push(d);
                scans.swap_remove(i);
            }
        }
    }
    let output = dists
        .iter()
        .tuple_combinations()
        .map(|([x1, y1, z1], [x2, y2, z2])| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
        .max()
        .unwrap();
    Ok(output)
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
        let file = File::open("assets/challenge19/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter_map(Result::ok)
            .map(|item| {
                if item == "" {
                    "\n\n".to_string()
                } else {
                    item + "\n"
                }
            })
            .collect::<String>();
        let input = input
            .split("\n\n")
            .map(|s| {
                s.lines()
                    .skip(1)
                    .filter(|item| !item.starts_with("---"))
                    .map(|l| {
                        let (a, tmp) = l.split_once(',').unwrap();
                        let (b, c) = tmp.split_once(',').unwrap();
                        [a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()]
                    })
                    .collect::<Vec<[i32; 3]>>()
            })
            .collect::<Vec<_>>();
        let output = part_one(input)?;
        assert_eq!(output, 79);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge19/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter_map(Result::ok)
            .map(|item| {
                if item == "" {
                    "\n\n".to_string()
                } else {
                    item + "\n"
                }
            })
            .collect::<String>();
        let input = input
            .split("\n\n")
            .map(|s| {
                s.lines()
                    .skip(1)
                    .filter(|item| !item.starts_with("---"))
                    .map(|l| {
                        let (a, tmp) = l.split_once(',').unwrap();
                        let (b, c) = tmp.split_once(',').unwrap();
                        [a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()]
                    })
                    .collect::<Vec<[i32; 3]>>()
            })
            .collect::<Vec<_>>();
        let output = part_two(input)?;
        assert_eq!(output, 3621);
        Ok(())
    }
}
