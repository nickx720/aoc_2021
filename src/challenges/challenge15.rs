use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengefifteen() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge15/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|s| s.ok())
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| (c as u8 - b'0') as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    let output = part_one(&input)?;
    println!("The output of first part is {}", output);
    let output = part_two(&input)?;
    println!("The output of second part is {}", output);
    Ok(())
}

fn shortest_path_bottom_corner(maze: &Vec<Vec<i32>>) -> i32 {
    let goal = (maze.len() - 1, maze[0].len() - 1);
    let mut dist = vec![vec![i32::MAX; maze[0].len()]; maze.len()];
    let mut q = BinaryHeap::new();
    q.push((0, 0, 0));
    while let Some((cost, x, y)) = q.pop() {
        if (x, y) == goal {
            return -cost;
        }
        if -cost > dist[x][y] {
            continue;
        }
        for (x1, y1) in [
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x, y.saturating_sub(1)),
            (x, y + 1),
        ] {
            if maze.get(x1).and_then(|row| row.get(y1)).is_none() {
                continue;
            }
            let next_cost = -cost + maze[x1][y1];
            if next_cost < dist[x1][y1] {
                q.push((-next_cost, x1, y1));
                dist[x1][y1] = next_cost;
            }
        }
    }
    unreachable!()
}

fn part_two(input: &Vec<Vec<i32>>) -> Result<i32, Box<dyn std::error::Error>> {
    let expanded = (0..(5 * input.len()))
        .map(|x| {
            (0..(5 * input[0].len()))
                .map(|y| {
                    let xlevel = (x / input.len()) as i32;
                    let ylevel = (y / input[0].len()) as i32;
                    let cost = input[x % input.len()][y % input[0].len()] + xlevel + ylevel;
                    if cost < 10 {
                        cost
                    } else {
                        cost - 9
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(shortest_path_bottom_corner(&expanded))
}

fn part_one(input: &Vec<Vec<i32>>) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(shortest_path_bottom_corner(&input))
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
        let file = File::open("assets/challenge15/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|s| s.ok())
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| (c as u8 - b'0') as i32)
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();
        let output = part_one(&input)?;
        assert_eq!(output, 40);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge15/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|s| s.ok())
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| (c as u8 - b'0') as i32)
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();
        let output = part_two(&input)?;
        assert_eq!(output, 315);
        Ok(())
    }
}
