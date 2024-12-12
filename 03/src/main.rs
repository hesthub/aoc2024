use regex::Regex;
use std::error::Error;
use std::fs;

fn part_one(input: String) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let inner_re = Regex::new(r"\d+").unwrap();

    let muls: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();

    let mut result: i32 = 0;

    for mul in muls {
        let numbers: Vec<&str> = inner_re.find_iter(mul).map(|m| m.as_str()).collect();

        if numbers.len() != 2 {
            println!("ERROR FOR MUL {mul}");
        }

        let sum: i32 = numbers[0].parse::<i32>().unwrap() * (numbers[1]).parse::<i32>().unwrap();

        result += sum;
    }
    result
}
fn part_two(input: String) -> i32 {
    let re = Regex::new(r"(mul\(\d+,\d+\)|(do\(\))|(don't\(\)))").unwrap();

    let inner_re = Regex::new(r"\d+").unwrap();

    let instructions: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();

    let mut result: i32 = 0;

    let mut enable: bool = true;

    for ins in instructions {
        match ins {
            "do()" => enable = true,
            "don't()" => enable = false,
            _ => {
                if enable {
                    let numbers: Vec<&str> = inner_re.find_iter(ins).map(|m| m.as_str()).collect();

                    if numbers.len() != 2 {
                        println!("ERROR FOR MUL {ins}");
                    }

                    let sum: i32 =
                        numbers[0].parse::<i32>().unwrap() * (numbers[1]).parse::<i32>().unwrap();

                    result += sum;
                }
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("./data/input")?;

    println!("Part 1: {}", part_one(input.clone()));
    println!("Part 2: {}", part_two(input.clone()));

    part_two(input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input: String = fs::read_to_string("./data/test").unwrap();

        let result = part_one(input);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_two() {
        let input: String = fs::read_to_string("./data/test").unwrap();

        let result = part_two(input);

        assert_eq!(result, 48);
    }
}
