use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengeleven() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge11/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|x| if let Ok(val) = x { val } else { "".to_string() })
        .collect::<Vec<String>>();
    //let output = part_one(input)?;
    //println!("The output of the first part is {}", output);
    let output = part_two(input)?;
    println!("The output of the second part is {}", output);
    Ok(())
}

fn part_one(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = input
        .iter()
        .map(|item| {
            item.split("")
                .filter(|&item| *item != "".to_string())
                .map(|each| if let Ok(item) = each.parse() { item } else { 0 })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut count = 0;
    for _ in 0..100 {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut mark = vec![vec![false; 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                input[i][j] += 1;
                if input[i][j] > 9 {
                    input[i][j] = 0;
                    mark[i][j] = true;
                    queue.push_back((i as i32, j as i32));
                    count += 1;
                }
            }
        }
        while let Some((i, j)) = queue.pop_front() {
            for (x, y) in vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (x, y) = (i + x, j + y);
                if x >= 0 && x < 10 && y >= 0 && y < 10 && !mark[x as usize][y as usize] {
                    input[x as usize][y as usize] += 1;
                    if input[x as usize][y as usize] > 9 {
                        input[x as usize][y as usize] = 0;
                        mark[x as usize][y as usize] = true;
                        queue.push_back((x, y));
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

fn part_two(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = input
        .iter()
        .map(|item| {
            item.split("")
                .filter(|&item| *item != "".to_string())
                .map(|each| if let Ok(item) = each.parse() { item } else { 0 })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    for step in 1.. {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut mark = vec![vec![false; 10]; 10];
        let mut count = 0;
        for i in 0..10 {
            for j in 0..10 {
                input[i][j] += 1;
                if input[i][j] > 9 {
                    input[i][j] = 0;
                    mark[i][j] = true;
                    queue.push_back((i as i32, j as i32));
                    count += 1;
                }
            }
        }
        while let Some((i, j)) = queue.pop_front() {
            for (x, y) in vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (x, y) = (i + x, j + y);
                if x >= 0 && x < 10 && y >= 0 && y < 10 && !mark[x as usize][y as usize] {
                    input[x as usize][y as usize] += 1;
                    if input[x as usize][y as usize] > 9 {
                        input[x as usize][y as usize] = 0;
                        mark[x as usize][y as usize] = true;
                        queue.push_back((x, y));
                        count += 1;
                    }
                }
            }
        }
        if count == 100 {
            return Ok(step);
        }
    }
    unreachable!()
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
        let file = File::open("assets/challenge11/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| if let Ok(val) = x { val } else { "".to_string() })
            .collect::<Vec<String>>();
        let output = part_one(input)?;
        assert_eq!(output, 1656);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge11/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| if let Ok(val) = x { val } else { "".to_string() })
            .collect::<Vec<String>>();
        let output = part_two(input)?;
        assert_eq!(output, 195);
        Ok(())
    }
}
