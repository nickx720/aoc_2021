use std::{collections::BinaryHeap, fs::read_to_string};

use hashbrown::HashMap;
use itertools::Itertools;

pub fn challengetwentythree() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("assets/challenge23/input.txt")?;
    //    let output = part_one(input)?;
    //    println!("The output of part one is {}", output);
    let output = part_two(input)?;
    println!("The output of part two is {}", output);
    Ok(())
}
fn right_configuration(maze: &Vec<Vec<char>>, size: usize) -> bool {
    maze[2..(2 + size)]
        .iter()
        .all(|l| itertools::equal(l[3..10].iter().copied(), "A#B#C#D".chars()))
}

fn moves(maze: &Vec<Vec<char>>, room_len: usize) -> Vec<(i64, Vec<Vec<char>>)> {
    let mut moves = Vec::new();
    for y in 0..maze[1].len() {
        // check moving into a room
        match maze[1][y] {
            'A'..='D' => {
                let (idx, exp) = match maze[1][y] {
                    'A' => (3, 1),
                    'B' => (5, 10),
                    'C' => (7, 100),
                    'D' => (9, 1000),
                    _ => unreachable!(),
                };
                let mut cost =
                    if y > idx && (idx..y).all(|c| maze[1][c] == '.') && maze[2][idx] == '.' {
                        y - idx
                    } else if y < idx
                        && ((y + 1)..=idx).all(|c| maze[1][c] == '.')
                        && maze[2][idx] == '.'
                    {
                        idx - y
                    } else {
                        continue;
                    };
                let mut m = maze.clone();
                let i = (2..=room_len)
                    .take_while(|&i| maze[i][idx] == '.')
                    .last()
                    .unwrap();
                if i != room_len && m[i + 1][idx] != maze[1][y] {
                    continue;
                }
                m[i][idx] = maze[1][y];
                m[1][y] = '.';
                cost += i - 1;
                moves.push(((cost * exp) as i64, m));
            }
            _ => {}
        }
    }
    for (x, y) in (2..=room_len).cartesian_product([3, 5, 7, 9]) {
        // check moving out of a room
        if (2..x).any(|i| maze[i][y] != '.') {
            continue;
        }
        if (x + 1..=room_len).any(|i| maze[i][y] == '.') {
            continue;
        }
        match maze[x][y] {
            'A'..='D' => {
                let exp = match maze[x][y] {
                    'A' => 1,
                    'B' => 10,
                    'C' => 100,
                    'D' => 1000,
                    _ => unreachable!(),
                };
                for i in y..maze[0].len() {
                    // move left
                    if maze[1][i] != '.' {
                        break;
                    }
                    if ![1, 2, 4, 6, 8, 10, 11].contains(&i) {
                        continue;
                    }
                    let cost = x - 1 + i - y;
                    let mut m = maze.clone();
                    m[1][i] = maze[x][y];
                    m[x][y] = '.';
                    moves.push(((cost * exp) as i64, m));
                }
                for i in (1..=y).rev() {
                    // move right
                    if maze[1][i] != '.' {
                        break;
                    }
                    if ![1, 2, 4, 6, 8, 10, 11].contains(&i) {
                        continue;
                    }
                    let cost = x - 1 + y - i;
                    let mut m = maze.clone();
                    m[1][i] = maze[x][y];
                    m[x][y] = '.';
                    moves.push(((cost * exp) as i64, m));
                }
            }
            _ => {}
        }
    }
    moves
}

fn shortest_path(maze: &Vec<Vec<char>>) -> i64 {
    let mut dist = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push((0, maze.clone()));
    while let Some((cost, m)) = q.pop() {
        if right_configuration(&m, maze.len() - 3) {
            return -cost;
        }
        if let Some(&c) = dist.get(&m) {
            if -cost > c {
                continue;
            }
        }
        for (nmoves, m) in moves(&m, maze.len() - 2) {
            let next_cost = -cost + nmoves;
            let &c = dist.get(&m).unwrap_or(&1000000);
            if c > next_cost {
                dist.insert(m.clone(), next_cost);
                q.push((-next_cost, m));
            }
        }
    }
    unreachable!()
}

fn part_one(input: String) -> Result<i64, Box<dyn std::error::Error>> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    Ok(shortest_path(&map))
}
fn part_two(input: String) -> Result<i64, Box<dyn std::error::Error>> {
    dbg!(&input);
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    map.insert(4, "  #D#C#B#A#  ".chars().collect());
    map.insert(4, "  #D#B#A#C#  ".chars().collect());
    Ok(shortest_path(&map))
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;
    #[test]
    fn part_one_works() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_to_string("assets/challenge23/sample.txt")?;
        let output = part_one(input)?;
        assert_eq!(output, 12521);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_to_string("assets/challenge23/sample.txt")?;
        let output = part_two(input)?;
        assert_eq!(output, 44169);
        Ok(())
    }
}
