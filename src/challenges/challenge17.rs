use itertools::Itertools;

pub fn challengeseventeen() -> Result<(), Box<dyn std::error::Error>> {
    let output = part_one((119, 176, -141, -84))?;
    println!("The output of first part is {}", output);
    let output = part_two((119, 176, -141, -84))?;
    println!("The output of second part is {}", output);
    Ok(())
}

fn try_vel(mut dx: i32, mut dy: i32, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> Option<i32> {
    let (mut x, mut y, mut maxy) = (0, 0, 0);
    loop {
        x += dx;
        y += dy;
        dx -= dx.signum();
        dy -= 1;
        if y > maxy {
            maxy = y;
        }
        match (xmin <= x && x <= xmax, ymin <= y && y <= ymax) {
            (true, true) => return Some(maxy),
            (false, _) if dx == 0 => return None,
            (_, false) if dy < 0 && y < ymin => return None,
            _ => {}
        }
    }
}

fn part_one(
    (xmin, xmax, ymin, ymax): (i32, i32, i32, i32),
) -> Result<i32, Box<dyn std::error::Error>> {
    let maxys = (0..=xmax)
        .cartesian_product(ymin..1000)
        .filter_map(|(x, y)| try_vel(x, y, xmin, xmax, ymin, ymax))
        .collect::<Vec<_>>();
    Ok(*maxys.iter().max().unwrap())
}

fn part_two(
    (xmin, xmax, ymin, ymax): (i32, i32, i32, i32),
) -> Result<usize, Box<dyn std::error::Error>> {
    let maxys = (0..=xmax)
        .cartesian_product(ymin..1000)
        .filter_map(|(x, y)| try_vel(x, y, xmin, xmax, ymin, ymax))
        .collect::<Vec<_>>();
    Ok(maxys.len())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_works() -> Result<(), Box<dyn std::error::Error>> {
        let output = part_one((20, 30, -10, -5))?;
        assert_eq!(output, 45);
        Ok(())
    }
    #[test]
    fn part_two_works() -> Result<(), Box<dyn std::error::Error>> {
        let output = part_two((20, 30, -10, -5))?;
        assert_eq!(output, 112);
        Ok(())
    }
}
