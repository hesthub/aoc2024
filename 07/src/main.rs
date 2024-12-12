use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Calibration {
    test: i64,
    equation: Vec<i64>,
}

fn test_calibration(cal: &Calibration, include_concat: bool) -> bool {
    if cal.equation.len() == 1 {
        return cal.test == cal.equation[0];
    }

    let base = if include_concat { 3 } else { 2 };

    let n = cal.equation.len() - 1;
    for mask in 0..(3u32.pow(n as u32)) {
        let mut result = cal.equation[0];
        for i in 0..n {
            let op = (mask / 3u32.pow(i as u32)) % base;
            match op {
                0 => result += cal.equation[i + 1],
                1 => result *= cal.equation[i + 1],
                2 if include_concat => result = concatenate(result, cal.equation[i + 1]),
                _ => continue,
            }
        }
        if result == cal.test {
            return true;
        }
    }
    false
}

fn concatenate(a: i64, b: i64) -> i64 {
    let b_str = b.to_string();
    let result_str = format!("{}{}", a, b_str);
    result_str.parse().unwrap()
}

fn read_calibrations(file_path: &str) -> Vec<Calibration> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let test = parts[0].trim().parse().expect("Failed to parse test value");
            let equation: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            Calibration { test, equation }
        })
        .collect()
}

fn part_one(calibrations: Vec<Calibration>) -> i64 {
    let mut result = 0;
    for calibration in calibrations {
        if test_calibration(&calibration,false) {
            result += calibration.test;
        }
    }
    result
}

fn part_two(calibrations: Vec<Calibration>) -> i64 {
    let mut result = 0;
    for calibration in calibrations {
        if test_calibration(&calibration, true) {
            result += calibration.test;
        }
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calibrations = read_calibrations("./data/input");

    println!("part 1: {}", part_one(calibrations.clone()));

    println!("part 2: {}", part_two(calibrations));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let calibrations = read_calibrations("./data/test");
        let result = part_one(calibrations);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two() {
        let calibrations = read_calibrations("./data/test");

        let result = part_two(calibrations);

        assert_eq!(result, 11387);
    }
}
