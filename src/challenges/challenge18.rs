use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengeeighteen() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge18/input.txt")?;
    let lines: Vec<Element> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|s| parse(&mut s.trim().chars()))
        .collect();
    let output = part_one(&lines)?;
    println!("The output of the first part is {}", output);
    let output = part_two(&lines)?;
    println!("The output of the second part is {}", output);
    Ok(())
}

#[derive(Debug, Clone)]
enum Element {
    Number(u32),
    Pair(Box<(Element, Element)>),
}

impl Element {
    fn magnitude(&self) -> u32 {
        match self {
            Element::Number(i) => *i,
            Element::Pair(p) => p.0.magnitude() * 3 + p.1.magnitude() * 2,
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            if self.try_explode(0).0 {
                continue;
            }
            if self.try_split() {
                continue;
            }
            break;
        }
        self
    }

    fn try_explode(&mut self, depth: i32) -> (bool, Option<u32>, Option<u32>) {
        match self {
            Element::Number(_) => (false, None, None),
            Element::Pair(p) => {
                if let Element::Number(l) = p.0 {
                    if let Element::Number(r) = p.1 {
                        if depth >= 4 {
                            *self = Element::Number(0);
                            return (true, Some(l), Some(r));
                        }
                    }
                }
                let (result, left, mut right) = p.0.try_explode(depth + 1);
                if result {
                    if right.is_some() {
                        p.1.inject_left(right.unwrap());
                        right = None;
                    }
                    return (true, left, right);
                }
                let (result, mut left, right) = p.1.try_explode(depth + 1);
                if result {
                    if left.is_some() {
                        p.0.inject_right(left.unwrap());
                        left = None;
                    }
                    return (true, left, right);
                }
                (false, None, None)
            }
        }
    }

    fn inject_left(&mut self, num: u32) {
        match self {
            Element::Number(ref mut l) => *l += num,
            Element::Pair(p) => p.0.inject_left(num),
        }
    }

    fn inject_right(&mut self, num: u32) {
        match self {
            Element::Number(ref mut r) => *r += num,
            Element::Pair(p) => p.1.inject_right(num),
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            Element::Number(n) => {
                if *n >= 10 {
                    let n = *n as f32;
                    *self = Element::Pair(Box::new((
                        Element::Number((n / 2.0).floor() as u32),
                        Element::Number((n / 2.0).ceil() as u32),
                    )));
                    true
                } else {
                    false
                }
            }
            Element::Pair(p) => p.0.try_split() || p.1.try_split(),
        }
    }
}

fn parse<T>(s: &mut T) -> Element
where
    T: Iterator<Item = char>,
{
    let mut elements = Vec::new();
    while let Some(c) = s.next() {
        match c {
            '[' => elements.push(parse(s)),
            c @ '0'..='9' => elements.push(Element::Number((c as u8 - b'0') as u32)),
            ',' => {}
            ']' => {
                return Element::Pair(Box::new((elements.swap_remove(0), elements.swap_remove(0))))
            }
            _ => unreachable!(),
        }
    }
    elements.swap_remove(0)
}

fn part_one(input: &[Element]) -> Result<u32, Box<dyn std::error::Error>> {
    let mut input = input.to_owned().into_iter();
    let mut element = Element::Pair(Box::new((
        input.next().unwrap().reduce(),
        input.next().unwrap().reduce(),
    )))
    .reduce();
    for e in input {
        element = Element::Pair(Box::new((element, e.reduce()))).reduce();
    }
    Ok(element.magnitude())
}

fn part_two(input: &[Element]) -> Result<u32, Box<dyn std::error::Error>> {
    let mut max = 0;
    for left in 0..input.len() {
        for right in 0..input.len() {
            if left == right {
                continue;
            }
            max = max.max(
                Element::Pair(Box::new((input[left].clone(), input[right].clone())))
                    .reduce()
                    .magnitude(),
            );
        }
    }
    Ok(max)
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
        let file = File::open("assets/challenge18/sample.txt")?;
        let lines: Vec<Element> = BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .map(|s| parse(&mut s.trim().chars()))
            .collect();
        let output = part_one(&lines)?;
        assert_eq!(output, 4140);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge18/sample.txt")?;
        let lines: Vec<Element> = BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .map(|s| parse(&mut s.trim().chars()))
            .collect();
        let output = part_two(&lines)?;
        assert_eq!(output, 3993);
        Ok(())
    }
}
