use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challengetwenty() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/challenge20/input.txt")?;
    let lines: Vec<Vec<i8>> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let algorithm = &lines[0];
    let map = &lines[2..]
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), *c))
                .collect::<Vec<_>>()
        })
        .collect::<Image>();
    let output = part_one(&algorithm, map)?;
    println!("The output of the first part is {}", output);
    let output = part_two(&algorithm, map)?;
    println!("The output of the second part is {}", output);
    Ok(())
}

type Image = HashMap<(isize, isize), i8>;

fn part_one(algorithm: &[i8], image: &Image) -> Result<usize, Box<dyn std::error::Error>> {
    let mut image = image.clone();
    for i in 0..2 {
        image = enhance(infinity(algorithm, i), algorithm, &image);
    }
    Ok(image.iter().filter(|(_, v)| **v == 1).count())
}

fn enhance(i: i8, algorithm: &[i8], image: &Image) -> Image {
    let mut output = Image::new();
    let min = image.iter().min_by_key(|(k, _)| *k).unwrap().0;
    let max = image.iter().max_by_key(|(k, _)| *k).unwrap().0;
    for y in (min.1 - 1)..=(max.1 + 1) {
        for x in (min.0 - 1)..=(max.0 + 1) {
            *output.entry((x, y)).or_default() = output_pixel(i, algorithm, image, (x, y))
        }
    }
    output
}

fn output_pixel(i: i8, algorithm: &[i8], image: &Image, pixel: (isize, isize)) -> i8 {
    let (x, y) = pixel;
    let mut value: isize = 0;
    for (x, y) in [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ] {
        value = (value << 1) | (image.get(&(x, y)).unwrap_or(&i) & 1) as isize;
    }
    algorithm[value as usize]
}

fn infinity(algorithm: &[i8], i: i8) -> i8 {
    match algorithm[0] {
        0 => 0,
        1 => i % 2,
        _ => unreachable!(),
    }
}

fn part_two(algorithm: &[i8], image: &Image) -> Result<usize, Box<dyn std::error::Error>> {
    let mut image = image.clone();
    for i in 0..50 {
        image = enhance(infinity(algorithm, i), algorithm, &image);
    }
    Ok(image.iter().filter(|(_, v)| **v == 1).count())
}
