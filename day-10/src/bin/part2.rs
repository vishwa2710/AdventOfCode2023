use geo::{coord, Contains, LineString, Polygon};
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
    let mut grid_map: HashMap<(usize, usize), Space> = HashMap::new();
    let mut start_row = 0;
    let mut start_col = 0;
    let grid_max_row = input_str.lines().count();
    let grid_max_col = input_str.lines().nth(0).unwrap().chars().count();
    for (row, line) in input_str.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                'S' => {
                    start_row = row;
                    start_col = col;
                    grid_map.insert(
                        (row, col),
                        Space {
                            direction_from: Direction::Up,
                            direction_to: Direction::Down,
                        },
                    );
                    continue;
                }
                _ch => {
                    grid_map.insert((row, col), *INPUT_MAP.get(&ch).unwrap());
                    continue;
                }
            };
        }
    }

    // gross, set the start position directions
    let mut directions = Vec::new();
    if start_row > 0 {
        if grid_map.contains_key(&(start_row - 1, start_col)) {
            let value = grid_map.get(&(start_row - 1, start_col)).unwrap();
            if (value.direction_from == Direction::Down) || (value.direction_to == Direction::Down)
            {
                directions.push(Direction::Up);
            }
        }
    }
    if start_row < grid_max_row - 1 {
        if grid_map.contains_key(&(start_row + 1, start_col)) {
            let value = grid_map.get(&(start_row + 1, start_col)).unwrap();
            if (value.direction_from == Direction::Up) || (value.direction_to == Direction::Up) {
                directions.push(Direction::Down);
            }
        }
    }
    if start_col > 0 {
        if grid_map.contains_key(&(start_row, start_col - 1)) {
            let value = grid_map.get(&(start_row, start_col - 1)).unwrap();
            if (value.direction_from == Direction::Right)
                || (value.direction_to == Direction::Right)
            {
                directions.push(Direction::Left);
            }
        }
    }
    if start_col < grid_max_col - 1 {
        if grid_map.contains_key(&(start_row, start_col + 1)) {
            let value = grid_map.get(&(start_row, start_col + 1)).unwrap();
            if (value.direction_from == Direction::Left) || (value.direction_to == Direction::Left)
            {
                directions.push(Direction::Right);
            }
        }
    }

    if directions.len() != 2 {
        panic!("Invalid start position");
    }

    let start_space: &mut Space = grid_map.get_mut(&(start_row, start_col)).unwrap();
    start_space.direction_from = directions[0];
    start_space.direction_to = directions[1];

    let mut current_coords = next_space(start_space.direction_to, start_row, start_col);

    let mut previous_direction = start_space.direction_to;

    let mut linestring = vec![(coord!(x: start_row as f64, y: start_col as f64))];
    while current_coords != (start_row, start_col) {
        let space = grid_map.get_mut(&current_coords).unwrap();

        // can go in either direction. pick one
        let correct_direction = {
            if space.direction_from == previous_direction.opposite() {
                space.direction_to
            } else {
                space.direction_from
            }
        };

        linestring.push(coord!(x: current_coords.0 as f64, y: current_coords.1 as f64));
        current_coords = next_space(correct_direction, current_coords.0, current_coords.1);
        previous_direction = correct_direction;
    }

    let linestring = LineString::new(linestring);
    let polygon = Polygon::new(linestring.clone(), vec![]);

    // print the grid
    let mut final_counter = 0;
    for i in 0..grid_max_row {
        for j in 0..grid_max_col {
            let coord = coord!(x: i as f64, y: j as f64);
            if linestring.contains(&coord) {
                continue;
            } else if polygon.contains(&coord) {
                final_counter += 1;
            }
        }
    }
    final_counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        );
        assert_eq!(result, "4");
    }

    #[test]
    fn it_works_2() {
        let result = solution(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, "8");
    }
}
