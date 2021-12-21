use std::{
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

use cached::{cached, Cached};
use rustc_hash::FxHashMap;

pub fn challengetwentyone() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge21/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(Result::ok)
        .map(|item| item.split(' ').last().unwrap().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let input: [u64; 2] = input.try_into().unwrap();
    //    let output = part_one(input)?;
    let output = *part_two(input[0], input[1], 0, 0).iter().max().unwrap();
    println!("The output of the second part is {}", output);
    Ok(())
}

fn roll(dice: &mut u64, rolls: &mut u64) -> u64 {
    *rolls += 1;
    let v = *dice;
    let mut d = *dice + 1;
    if d == 101 {
        d = 1;
    }
    *dice = d;
    v
}

fn part_one(mut pos: [u64; 2]) -> Result<u64, Box<dyn std::error::Error>> {
    let mut rolls = 0;
    let mut scores = [0, 0];
    let mut dice = 1;
    loop {
        let a = roll(&mut dice, &mut rolls);
        let b = roll(&mut dice, &mut rolls);
        let c = roll(&mut dice, &mut rolls);
        pos[0] += (a + b + c) % 10;
        if pos[0] > 10 {
            pos[0] -= 10;
        }
        debug_assert!(pos[0] >= 1 && pos[0] <= 10);
        scores[0] += pos[0];
        if scores[0] >= 1000 {
            break;
        }
        let a = roll(&mut dice, &mut rolls);
        let b = roll(&mut dice, &mut rolls);
        let c = roll(&mut dice, &mut rolls);
        pos[1] += (a + b + c) % 10;
        if pos[1] > 10 {
            pos[1] -= 10;
        }
        debug_assert!(pos[1] >= 1 && pos[1] <= 10);
        scores[1] += pos[1];
        if scores[1] >= 1000 {
            break;
        }
    }
    Ok(rolls * *scores.iter().min().unwrap())
}

#[derive(Debug, Default)]
struct FxCache<K: Hash + Eq, V> {
    store: FxHashMap<K, V>,
}

impl<K: Hash + Eq, V> Cached<K, V> for FxCache<K, V> {
    fn cache_get(&mut self, k: &K) -> Option<&V> {
        self.store.get(k)
    }
    fn cache_get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.store.get_mut(k)
    }
    fn cache_get_or_set_with<F: FnOnce() -> V>(&mut self, k: K, f: F) -> &mut V {
        self.store.entry(k).or_insert_with(f)
    }
    fn cache_set(&mut self, k: K, v: V) -> Option<V> {
        self.store.insert(k, v)
    }
    fn cache_remove(&mut self, k: &K) -> Option<V> {
        self.store.remove(k)
    }
    fn cache_clear(&mut self) {
        self.store.clear();
    }
    fn cache_reset(&mut self) {
        self.store = FxHashMap::default();
    }
    fn cache_size(&self) -> usize {
        self.store.len()
    }
}

cached! {
    PART2: FxCache<(u64,u64,u64,u64),[u64;2]> = FxCache::default();

    fn part_two(current_player:u64, other_player:u64,current_score:u64,other_score:u64)-> [u64;2] = {
        const SCORES:[u64;27] = [
            3,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6,7,7,7,7,7,7,8,8,8,9
        ];
        let mut wins = [0;2];
        for s in SCORES{
            let mut current_player = current_player;
            let mut current_score = current_score;
            current_player +=s;
            if current_player > 10 {
                current_player -=10;
            }
            current_score +=current_player;
            if current_score >= 21 {
                wins[0] +=1;
            } else {
                let sub_win = part_two(other_player,current_player,other_score,current_score);
                wins[1] +=sub_win[0];
                wins[0] +=sub_win[1];
            }
        }
        wins
    }
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
        let file = File::open("assets/challenge21/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter_map(Result::ok)
            .map(|item| item.split(' ').last().unwrap().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let input: [u64; 2] = input.try_into().unwrap();
        let output = part_one(input)?;
        assert_eq!(output, 739785);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge21/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .filter_map(Result::ok)
            .map(|item| item.split(' ').last().unwrap().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let input: [u64; 2] = input.try_into().unwrap();
        let output = *part_two(input[0], input[1], 0, 0).iter().max().unwrap();
        assert_eq!(output, 444356092776315);
        Ok(())
    }
}
