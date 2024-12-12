use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part_one(file: File) -> i32 {
    let reader = BufReader::new(file);

    let mut safe = 0;

    for line in reader.lines() {
        let levels: Vec<u16> = line
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();

        if check_levels(&levels).0 == 0 {
            safe += 1;
        }
    }

    safe
}
fn part_two(file: File) -> i32 {
    let reader = BufReader::new(file);

    let mut safe = 0;

    for line in reader.lines() {
        let levels: Vec<u16> = line
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();

        if is_safe_with_dampener(&levels) {
            safe += 1;
        }
    }

    safe
}
fn is_safe_with_dampener(levels: &Vec<u16>) -> bool {
    if check_levels(levels).0 == 0 {
        return true;
    }

    for i in 0..levels.len() {
        let mut corrected_lvl = levels.clone();
        corrected_lvl.remove(i);
        if check_levels(&corrected_lvl).0 == 0 {
            return true;
        }
    }

    false
}
fn check_levels(levels: &[u16]) -> (u16, usize) {
    let increasing: bool = levels[1] > levels[0];

    let mut unsafe_index = 0;
    let mut nr_of_unsafe = 0;

    for i in 1..levels.len() {
        let abs = levels[i - 1].abs_diff(levels[i]);

        if (increasing && levels[i] < levels[i - 1]) || (!increasing && levels[i] > levels[i - 1]) {
            nr_of_unsafe += 1;
            unsafe_index = i;
            break;
        }

        if abs > 3 || abs == 0 {
            nr_of_unsafe += 1;
            unsafe_index = i;
            break;
        }
    }

    (nr_of_unsafe, unsafe_index)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(Path::new("./data/input"))?;

    println!("{}", part_one(file.try_clone().unwrap()));
    println!("{}", part_two(file));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let file = File::open(Path::new("./data/test")).unwrap();

        let result = part_one(file);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let file = File::open(Path::new("./data/test")).unwrap();

        let result = part_two(file);

        assert_eq!(result, 4);
    }
}
