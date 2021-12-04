use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengefour() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge4/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|x| x.ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    // let output = part_one(input)?;
    // println!("The output of challenge 4 first part is {}", output);
    let output = part_two(input)?;
    println!("The output of challenge 4 second part is {}", output);
    Ok(())
}

type Board = Vec<Vec<usize>>;

fn part_one(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
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
    let boards = &input[1..]
        .join("\n")
        .trim()
        .split("\n\n")
        .into_iter()
        .map(|b| {
            b.lines()
                .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
                .collect()
        })
        .collect::<Vec<Board>>();
    for i in 5..draws.len() {
        let winner = boards.iter().find_map(|b| check_board(&draws[0..i], &b));
        if let Some(score) = winner {
            return Ok(score);
        }
    }
    unreachable!()
}

fn part_two(input: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
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
    let mut boards = input[1..]
        .join("\n")
        .trim()
        .split("\n\n")
        .into_iter()
        .map(|b| {
            b.lines()
                .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
                .collect()
        })
        .collect::<Vec<Board>>()
        .into_iter()
        .collect::<HashSet<_>>();
    for i in 5..draws.len() {
        let winners = boards
            .iter()
            .filter_map(|b| check_board(&draws[0..i], &b).map(|score| (b.clone(), score)))
            .collect::<Vec<_>>();
        for (b, _) in &winners {
            boards.remove(b);
        }
        if boards.is_empty() {
            return Ok(winners[0].1);
        }
    }
    unreachable!()
}

fn calculate_board(draws: &[usize], board: &Board) -> usize {
    board.iter().flatten().filter(|&x| !draws.contains(x)).sum()
}

fn check_board(draws: &[usize], board: &Board) -> Option<usize> {
    for i in 0..5 {
        if (0..5).all(|j| draws.contains(&board[i][j])) {
            return Some(calculate_board(draws, board) * draws.last().unwrap());
        }
        if (0..5).all(|j| draws.contains(&board[j][i])) {
            return Some(calculate_board(draws, board) * draws.last().unwrap());
        }
    }
    None
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

    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge4/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|x| x.ok())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>();
        let output = part_two(input)?;
        assert_eq!(output, 1924);
        Ok(())
    }
}
