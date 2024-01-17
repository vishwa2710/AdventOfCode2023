use std::collections::HashSet;

fn main() {
    // let result = solution(include_str!("input.txt"));
    // println!("{}", result);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Turn {
    Left,
    Right,
}

impl Direction {
    fn take_turn(current_direction: Direction, turn: Turn) -> Direction {
        match current_direction {
            Direction::Up => match turn {
                Turn::Left => Direction::Left,
                Turn::Right => Direction::Right,
            },
            Direction::Down => match turn {
                Turn::Left => Direction::Right,
                Turn::Right => Direction::Left,
            },
            Direction::Left => match turn {
                Turn::Left => Direction::Down,
                Turn::Right => Direction::Up,
            },
            Direction::Right => match turn {
                Turn::Left => Direction::Up,
                Turn::Right => Direction::Down,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    values: Vec<Vec<u32>>,
    max_rows: usize,
    max_cols: usize,
}

impl Grid {
    fn new(values: Vec<Vec<u32>>) -> Grid {
        Grid {
            values: values.clone(),
            max_rows: values.len() as usize,
            max_cols: values[0].len() as usize,
        }
    }

    fn get_next_position(
        &self,
        position: &(usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Down => {
                if position.0 >= self.max_rows - 1 {
                    None
                } else {
                    Some((position.0 + 1, position.1))
                }
            }
            Direction::Up => {
                if position.0 <= 0 {
                    None
                } else {
                    Some((position.0 - 1, position.1))
                }
            }
            Direction::Right => {
                if position.1 >= self.max_cols - 1 {
                    None
                } else {
                    Some((position.0, position.1 + 1))
                }
            }
            Direction::Left => {
                if position.1 <= 0 {
                    None
                } else {
                    Some((position.0, position.1 - 1))
                }
            }
        }
    }

    fn get_valid_directions(
        &self,
        current_position: (usize, usize),
        current_direction: Direction,
    ) -> Vec<Direction> {
        let mut valid_directions = vec![];

        if current_position.0 > 0 && current_direction != Direction::Down {
            valid_directions.push(Direction::Up);
        }

        if current_position.0 < self.max_rows as usize - 1 && current_direction != Direction::Up {
            valid_directions.push(Direction::Down);
        }

        if current_position.1 > 0 && current_direction != Direction::Right {
            valid_directions.push(Direction::Left);
        }

        if current_position.1 < self.max_cols as usize - 1 && current_direction != Direction::Left {
            valid_directions.push(Direction::Right);
        }

        valid_directions
    }
}

fn solution(input_str: &str) -> String {
    let grid = Grid::new(
        input_str
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>(),
    );

    let mut current_position = (grid.max_rows - 1, grid.max_cols - 1);
    let mut current_direction = Direction::Up; // Can be Up or Left. Doesn't matter.

    while current_position != (0, 0) {
        let current_cost = grid.values[current_position.0][current_position.1];

        let valid_directions = grid.get_valid_directions(current_position, current_direction);

        let mut next_position = None;

        for direction in valid_directions {}

        current_position = next_position.unwrap();
    }
    "104".to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(result, "51");
    }
}
