use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_input(path: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    let file = File::open(Path::new(path)).expect("no file found");

    let reader = BufReader::new(file);

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let lines = reader.lines();

    for line in lines {
        let l = line?;
        let parts: Vec<&str> = l.split("   ").collect();
        if parts.len() == 2 {
            left.push(parts[0].trim().parse().unwrap());
            right.push(parts[1].trim().parse().unwrap());
        }
    }

    Ok((left, right))
}

fn part_one(mut left: Vec<u32>, mut right: Vec<u32>) -> u32{
    left.sort();
    right.sort();
    let mut result: Vec<u32> = Vec::new();
    for i in 0..left.len() {
        let l = left[i];
        let r = right[i];

        if l > r {
            result.push(l - r)
        } else {
            result.push(r - l)
        }
    }
    result.iter().sum::<u32>()
}

fn part_two(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort();
    right.sort();

    let mut v2: Vec<u32> = Vec::new();

    for num in left {
        v2.push(num * right.iter().filter(|&n| *n == num).count() as u32);
        right.iter().filter(|&n| *n == num).count();
    }

    v2.iter().sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = read_input("./data/input")?;

    part_one(left.clone(), right.clone());

    part_two(left, right);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {

        let (left, right) = read_input("./data/test").unwrap();
        
        let result = part_one(left,right);
        
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {

        let (left, right) = read_input("./data/test").unwrap();

        let result = part_two(left,right);

        assert_eq!(result, 31);
    }
}
