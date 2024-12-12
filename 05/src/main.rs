use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone)]
struct Rules {
    data: Vec<(isize, isize)>,
}

impl Rules {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(Path::new(path)).expect("no file found");
        let reader = BufReader::new(file);

        let data: Vec<(isize, isize)> = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let parts: Vec<&str> = line.split('|').collect();
                (parts[0].parse().unwrap(), parts[1].parse().unwrap())
            })
            .collect();

        Ok(Rules { data })
    }

    fn validate(&self, page: &[isize]) -> bool {
        for &(first, second) in &self.data {
            let first_index = page.iter().position(|&x| x == first);
            let second_index = page.iter().position(|&x| x == second);
            if let (Some(fi), Some(si)) = (first_index, second_index) {
                if fi > si {
                    return false;
                }
            }
        }
        true
    }

    fn apply_rule<'a>(&self, page: &'a mut Vec<isize>) -> &'a mut Vec<isize> {
        let mut changed = true;
        while changed {
            changed = false;
            for &(first, second) in &self.data {
                let first_index = page.iter().position(|&x| x == first);
                let second_index = page.iter().position(|&x| x == second);
                if let (Some(fi), Some(si)) = (first_index, second_index) {
                    if fi > si {
                        page.swap(fi, si);
                        changed = true;
                    }
                }
            }
        }
        page
    }
}

#[derive(Clone)]
struct Pages {
    data: Vec<Vec<isize>>,
}

impl Pages {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(Path::new(path)).expect("no file found");
        let reader = BufReader::new(file);

        let mut data: Vec<Vec<isize>> = vec![];

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<isize> = line
                .split(',')
                .filter_map(|num| num.trim().parse::<isize>().ok())
                .collect();
            data.push(numbers)
        }

        Ok(Pages { data })
    }

    fn get_mid_value_sum(&self) -> isize {
        let mut result = 0;
        for datum in &self.data {
            result += datum[datum.len() / 2]
        }
        result
    }
}

impl fmt::Display for Pages {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, datum) in self.data.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(
                f,
                "page: {}",
                datum
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, (first, second)) in self.data.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "Rule: {}|{}", first, second)?;
        }
        Ok(())
    }
}

fn part_one(rules: Rules, pages: Pages) -> isize {
    let valid = find_pages(&rules, pages, true);
    valid.get_mid_value_sum()
}

fn part_two(rules: Rules, pages: Pages) -> isize {
    let invalid_pages: Vec<Vec<isize>> = pages
        .data
        .into_iter()
        .filter(|page| !rules.validate(page))
        .collect();

    let mut corrected_pages: Vec<Vec<isize>> = Vec::new();

    for mut page in invalid_pages {
        rules.apply_rule(&mut page);
        corrected_pages.push(page);
    }

    let corrected = Pages {
        data: corrected_pages,
    };

    corrected.get_mid_value_sum()
}

fn find_pages(rules: &Rules, pages: Pages, valid: bool) -> Pages {
    let mut valid_pages: Vec<Vec<isize>> = vec![];

    for page in pages.data {
        if rules.validate(&page) == valid {
            valid_pages.push(page.clone());
        }
    }

    Pages { data: valid_pages }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rules = Rules::from_file("./data/rules").unwrap();

    let pages = Pages::from_file("./data/input").unwrap();

    println!("{}", part_one(rules.clone(), pages.clone()));
    println!("{}", part_two(rules, pages));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let rules = Rules::from_file("./data/testrules").unwrap();

        let pages = Pages::from_file("./data/test").unwrap();

        let result = part_one(rules, pages);

        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_two() {
        let rules = Rules::from_file("./data/testrules").unwrap();

        let pages = Pages::from_file("./data/test").unwrap();

        let result = part_two(rules, pages);

        assert_eq!(result, 123);
    }
}
