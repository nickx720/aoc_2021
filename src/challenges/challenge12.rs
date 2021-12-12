use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengetweleve() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge12/input.txt")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|item| {
            if let Ok(new_item) = item {
                new_item
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<String>>();
    let mut graph = HashMap::new();
    for item in input.iter() {
        let (a, b) = item.split_once("-").unwrap();
        graph.entry(a).or_insert(Vec::new()).push(b);
        graph.entry(b).or_insert(Vec::new()).push(a);
    }
    let output = num_paths(&graph, "start", &mut Vec::new(), true);
    println!("The first part output is {}", output);
    let output = num_paths(&graph, "start", &mut Vec::new(), false);
    println!("The second part output is {}", output);
    Ok(())
}

fn num_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    src: &'a str,
    path: &mut Vec<&'a str>,
    mut seen_twice: bool,
) -> usize {
    if src == "end" {
        return 1;
    }
    if src.chars().all(|c| c.is_lowercase()) && path.contains(&src) {
        if seen_twice || src == "start" {
            return 0;
        }
        seen_twice = true;
    }
    path.push(src);
    let ans = graph[src]
        .iter()
        .map(|src| num_paths(graph, src, path, seen_twice))
        .sum();
    path.pop();
    ans
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;
    #[test]
    fn part_one_works() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("assets/challenge12/sample.txt")?;
        let reader = BufReader::new(file);
        let input = reader
            .lines()
            .map(|item| {
                if let Ok(new_item) = item {
                    new_item
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<String>>();
        let mut graph = HashMap::new();
        for item in input.iter() {
            let (a, b) = item.split_once("-").unwrap();
            graph.entry(a).or_insert(Vec::new()).push(b);
            graph.entry(b).or_insert(Vec::new()).push(a);
        }
        let output = num_paths(&graph, "start", &mut Vec::new(), true);
        assert_eq!(output, 226);
        Ok(())
    }
}
