use phf::{phf_map, Map};
use std::collections::HashMap;

const INPUT_MAP: Map<char, Space> = phf_map! {
    '|' => Space {direction_from: Direction::Up, direction_to: Direction::Down},
    '-' => Space {direction_from: Direction::Left, direction_to: Direction::Right},
    'L' => Space {direction_from: Direction::Up, direction_to: Direction::Right},
    'J' => Space {direction_from: Direction::Up, direction_to: Direction::Left},
    '7' => Space {direction_from: Direction::Down, direction_to: Direction::Left},
    'F' => Space {direction_from: Direction::Down, direction_to: Direction::Right},
};

fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Space {
    direction_from: Direction,
    direction_to: Direction,
}

fn next_space(direction: Direction, row: usize, col: usize) -> (usize, usize) {
    match direction {
        Direction::Up => (row.checked_sub(1).unwrap(), col),
        Direction::Down => (row.checked_add(1).unwrap(), col),
        Direction::Left => (row, col.checked_sub(1).unwrap()),
        Direction::Right => (row, col.checked_add(1).unwrap()),
    }
}

fn solution(input_str: &str) -> String {
    let mut grid: HashMap<(usize, usize), Space> = HashMap::new();
    let mut start_row = 0;
    let mut start_col = 0;
    let mut grid_max_row = 0;
    let mut grid_max_col = 0;
    for (row, line) in input_str.lines().enumerate() {
        grid_max_row = grid_max_row.max(row);
        for (col, ch) in line.chars().enumerate() {
            grid_max_col = grid_max_col.max(col);
            match ch {
                '.' => continue,
                'S' => {
                    start_row = row;
                    start_col = col;
                    grid.insert(
                        (row, col),
                        Space {
                            direction_from: Direction::Up,
                            direction_to: Direction::Down,
                        },
                    );
                    continue;
                }
                _ch => {
                    grid.insert((row, col), *INPUT_MAP.get(&ch).unwrap());
                    continue;
                }
            };
        }
    }

    // gross, set the start position directions
    let mut directions = Vec::new();
    if start_row > 0 {
        if grid.contains_key(&(start_row - 1, start_col)) {
            let value = grid.get(&(start_row - 1, start_col)).unwrap();
            if (value.direction_from == Direction::Down) || (value.direction_to == Direction::Down)
            {
                directions.push(Direction::Up);
            }
        }
    }
    if start_row < grid_max_row - 1 {
        if grid.contains_key(&(start_row + 1, start_col)) {
            let value = grid.get(&(start_row + 1, start_col)).unwrap();
            if (value.direction_from == Direction::Up) || (value.direction_to == Direction::Up) {
                directions.push(Direction::Down);
            }
        }
    }
    if start_col > 0 {
        if grid.contains_key(&(start_row, start_col - 1)) {
            let value = grid.get(&(start_row, start_col - 1)).unwrap();
            if (value.direction_from == Direction::Right)
                || (value.direction_to == Direction::Right)
            {
                directions.push(Direction::Left);
            }
        }
    }
    if start_col < grid_max_col - 1 {
        if grid.contains_key(&(start_row, start_col + 1)) {
            let value = grid.get(&(start_row, start_col + 1)).unwrap();
            if (value.direction_from == Direction::Left) || (value.direction_to == Direction::Left)
            {
                directions.push(Direction::Right);
            }
        }
    }

    if directions.len() != 2 {
        panic!("Invalid start position");
    }

    let start_space: &mut Space = grid.get_mut(&(start_row, start_col)).unwrap();
    start_space.direction_from = directions[0];
    start_space.direction_to = directions[1];
    println!("{:?} {} {}", start_space, start_row, start_col);

    let mut counter = 1;
    let mut current_coords = next_space(start_space.direction_to, start_row, start_col);

    let mut previous_direction = start_space.direction_to;
    while current_coords != (start_row, start_col) {
        let space = grid.get_mut(&current_coords).unwrap();

        // can go in either direction. pick one
        let correct_direction = {
            if space.direction_from == previous_direction.opposite() {
                space.direction_to
            } else {
                space.direction_from
            }
        };

        current_coords = next_space(correct_direction, current_coords.0, current_coords.1);
        counter += 1;
        previous_direction = correct_direction;
    }

    println!("{}", counter);

    (counter / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn it_works_2() {
        let result = solution(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, "4");
    }
}
