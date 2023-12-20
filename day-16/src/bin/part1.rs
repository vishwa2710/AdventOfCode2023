use std::collections::HashSet;

fn main() {
    let result = solution(include_str!("input.txt"));
    println!("{}", result);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Grid {
    values: Vec<Vec<char>>,
    covered: Vec<Vec<bool>>,
    max_rows: isize,
    max_cols: isize,
}

impl Grid {
    fn new(values: Vec<Vec<char>>) -> Grid {
        Grid {
            values: values.clone(),
            covered: vec![vec![false; values[0].len()]; values.len()],
            max_rows: values.len() as isize,
            max_cols: values[0].len() as isize,
        }
    }

    fn get_positions(
        &self,
        position: &(isize, isize),
        direction: &Direction,
    ) -> Vec<(isize, isize)> {
        let mut position = *position;
        let mut positions = Vec::new();

        loop {
            match direction {
                Direction::Down => {
                    if position.0 == self.max_rows - 1 {
                        break;
                    }
                    position = (position.0 + 1, position.1);
                }
                Direction::Up => {
                    if position.0 == 0 {
                        break;
                    }
                    position = (position.0 - 1, position.1);
                }
                Direction::Right => {
                    if position.1 == self.max_cols - 1 {
                        break;
                    }
                    position = (position.0, position.1 + 1);
                }
                Direction::Left => {
                    if position.1 == 0 {
                        break;
                    }
                    position = (position.0, position.1 - 1);
                }
            }
            positions.push(position);
            if self.values[position.0 as usize][position.1 as usize] != '.' {
                break;
            }
        }
        positions
    }

    fn get_next_direction(
        &self,
        position: &(isize, isize),
        direction: &Direction,
    ) -> Vec<Direction> {
        match self.values[position.0 as usize][position.1 as usize] {
            '\\' => match direction {
                Direction::Down => vec![Direction::Right],
                Direction::Up => vec![Direction::Left],
                Direction::Right => vec![Direction::Down],
                Direction::Left => vec![Direction::Up],
            },
            '/' => match direction {
                Direction::Down => vec![Direction::Left],
                Direction::Up => vec![Direction::Right],
                Direction::Right => vec![Direction::Up],
                Direction::Left => vec![Direction::Down],
            },
            '|' => match direction {
                Direction::Up => vec![Direction::Up],
                Direction::Down => vec![Direction::Down],
                Direction::Left => vec![Direction::Down, Direction::Up],
                Direction::Right => vec![Direction::Down, Direction::Up],
            },
            '-' => match direction {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Left],
                Direction::Right => vec![Direction::Right],
            },
            _ => vec![],
        }
    }
}

fn solve(
    position: (isize, isize),
    direction: Direction,
    grid: &mut Grid,
    move_set: &mut HashSet<((isize, isize), Direction)>,
) {
    let positions = grid.get_positions(&position, &direction);

    positions
        .iter()
        .for_each(|(x, y)| grid.covered[*x as usize][*y as usize] = true);

    let new_position = positions.last().unwrap_or(&position);
    let new_directions = grid.get_next_direction(&new_position, &direction);

    if !move_set.insert((*new_position, direction.clone())) {
        return;
    }

    for new_direction in new_directions {
        solve(*new_position, new_direction, grid, move_set);
    }
}

fn solution(input_str: &str) -> String {
    let mut grid = Grid::new(
        input_str
            .lines()
            .map(|line| line.chars().map(|ch| ch).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );

    let current_position: (isize, isize) = (0, -1);
    let current_direction = Direction::Right;
    let mut move_set = HashSet::new();

    solve(
        current_position,
        current_direction,
        &mut grid,
        &mut move_set,
    );

    grid.covered
        .iter()
        .map(|line| line.iter().map(|x| *x as isize).sum::<isize>())
        .sum::<isize>()
        .to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, "46");
    }
}
