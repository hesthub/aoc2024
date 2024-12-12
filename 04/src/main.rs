use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const XMAS_DIAGONALS: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

pub struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    pub fn from_reader(reader: BufReader<File>) -> Result<Self, std::io::Error> {
        let data: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        Ok(Self { data })
    }

    pub fn height(&self) -> isize {
        self.data.len() as isize
    }

    pub fn width(&self) -> isize {
        self.data[0].len() as isize
    }

    pub fn get(&self, x: isize, y: isize) -> Option<char> {
        self.data.get(x as usize)?.get(y as usize).copied()
    }
}
pub fn part_one(grid: &Grid) -> usize {
    let mut count = 0;
    for x in 0..grid.height() {
        for y in 0..grid.width() {
            if grid.get(x, y) == Some('X') {
                count += DIRECTIONS
                    .iter()
                    .filter(|&&(dx, dy)| check_word(grid, x, y, dx, dy))
                    .count();
            }
        }
    }
    count
}

pub fn part_two(grid: &Grid) -> usize {
    let mut count = 0;
    for x in 0..grid.height() {
        for y in 0..grid.width() {
            if grid.get(x, y) == Some('A') {
                count += check_xmas(grid, x, y) as usize;
            }
        }
    }
    count
}
fn check_xmas(grid: &Grid, x: isize, y: isize) -> bool {
    let mut count = 0;

    let top_left = (x + XMAS_DIAGONALS[0].0, y + XMAS_DIAGONALS[0].1);
    let top_right = (x + XMAS_DIAGONALS[1].0, y + XMAS_DIAGONALS[1].1);
    let bot_left = (x + XMAS_DIAGONALS[2].0, y + XMAS_DIAGONALS[2].1);
    let bot_right = (x + XMAS_DIAGONALS[3].0, y + XMAS_DIAGONALS[3].1);

    if grid.get(top_left.0, top_left.1) == Some('M')
        && grid.get(bot_right.0, bot_right.1) == Some('S')
    {
        count += 1;
    }

    if grid.get(top_left.0, top_left.1) == Some('S')
        && grid.get(bot_right.0, bot_right.1) == Some('M')
    {
        count += 1;
    }

    if grid.get(bot_left.0, bot_left.1) == Some('M')
        && grid.get(top_right.0, top_right.1) == Some('S')
    {
        count += 1;
    }

    if grid.get(bot_left.0, bot_left.1) == Some('S')
        && grid.get(top_right.0, top_right.1) == Some('M')
    {
        count += 1;
    }

    count >= 2
}

fn check_word(grid: &Grid, x: isize, y: isize, dx: isize, dy: isize) -> bool {
    "XMAS".chars().enumerate().all(|(k, c)| {
        let nx = x + k as isize * dx;
        let ny = y + k as isize * dy;
        grid.get(nx, ny) == Some(c)
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(Path::new("./data/input")).expect("no file found");
    let reader = BufReader::new(file);

    let grid: Grid = Grid::from_reader(reader).unwrap();

    println!("{}", part_one(&grid));
    println!("{}", part_two(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let file = File::open(Path::new("./data/test")).expect("no file found");
        let reader = BufReader::new(file);

        let grid: Grid = Grid::from_reader(reader).unwrap();

        let result = part_one(&grid);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_two() {
        let file = File::open(Path::new("./data/test")).expect("no file found");
        let reader = BufReader::new(file);

        let grid: Grid = Grid::from_reader(reader).unwrap();

        let result = part_two(&grid);

        assert_eq!(result, 9);
    }
}
