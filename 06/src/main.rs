use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn delta(&self) -> (isize, isize) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    data: Vec<Vec<char>>,
    guard_pos: (isize, isize),
    direction: Direction,
}

impl Grid {
    pub fn from_reader(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(Path::new(file))?;
        let reader = BufReader::new(file);
        let data: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();

        let mut guard_pos = (0, 0);
        let mut direction = Direction::Up;

        for (x, row) in data.iter().enumerate() {
            for (y, &char) in row.iter().enumerate() {
                match char {
                    '^' => {
                        guard_pos = (x as isize, y as isize);
                        direction = Direction::Up;
                    }
                    '>' => {
                        guard_pos = (x as isize, y as isize);
                        direction = Direction::Right;
                    }
                    'v' => {
                        guard_pos = (x as isize, y as isize);
                        direction = Direction::Down;
                    }
                    '<' => {
                        guard_pos = (x as isize, y as isize);
                        direction = Direction::Left;
                    }
                    _ => {}
                }
            }
        }

        Ok(Self {
            data,
            guard_pos,
            direction,
        })
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

    pub fn get_distinct(&mut self) -> i32 {
        let mut result = 0;
        for row in self.data.iter() {
            for &char in row.iter() {
                if char == 'X' {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn move_guard(&mut self) -> bool {
        let (dx, dy) = self.direction.delta();
        let new_pos = (self.guard_pos.0 + dx, self.guard_pos.1 + dy);

        if let Some(tile) = self.get(new_pos.0, new_pos.1) {
            if tile == '#' {
                self.direction.turn_right();
                true
            } else {
                self.data[self.guard_pos.0 as usize][self.guard_pos.1 as usize] = 'X';
                self.guard_pos = new_pos;
                true
            }
        } else {
            self.data[self.guard_pos.0 as usize][self.guard_pos.1 as usize] = 'X';
            false
        }
    }

    pub fn count_visited(&self) -> usize {
        self.data
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == 'X')
            .count()
    }

    pub fn find_loop_positions(&self) -> usize {
        let mut loop_count = 0;
        for x in 0..self.height() {
            for y in 0..self.width() {
                if self.get(x, y) == Some('.') && (x, y) != self.guard_pos {
                    if self.check_loop_with_obstacle(x, y) {
                        loop_count += 1;
                    }
                }
            }
        }
        loop_count
    }

    fn check_loop_with_obstacle(&self, obstacle_x: isize, obstacle_y: isize) -> bool {
        let mut current_pos = self.guard_pos;
        let mut current_dir = self.direction;
        let mut visited = HashSet::new();
        let mut steps = 0;
        let max_steps = self.height() * self.width() * 4; // Arbitrary large number

        while steps < max_steps {
            if !visited.insert((current_pos, current_dir)) {
                return true; // Loop detected
            }

            let (dx, dy) = current_dir.delta();
            let new_pos = (current_pos.0 + dx, current_pos.1 + dy);

            if new_pos == (obstacle_x, obstacle_y) || self.get(new_pos.0, new_pos.1) == Some('#') {
                current_dir.turn_right();
            } else if self.get(new_pos.0, new_pos.1).is_none() {
                return false; // Guard left the map
            } else {
                current_pos = new_pos;
            }

            steps += 1;
        }

        false
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = Grid::from_reader("./data/input")?;

    let p2_grid = grid.clone();

    while grid.move_guard() {}

    println!("part one: {}", grid.get_distinct());

    println!("part two: {}", p2_grid.find_loop_positions());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut grid = Grid::from_reader("./data/test").unwrap();

        while grid.move_guard() {}

        let result = grid.get_distinct();

        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::from_reader("./data/test").unwrap();

        let result = grid.find_loop_positions();

        assert_eq!(result, 6);
    }
}
