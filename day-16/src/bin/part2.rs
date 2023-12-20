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

    fn get_next_position(
        &self,
        position: &(isize, isize),
        direction: &Direction,
    ) -> Option<(isize, isize)> {
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
            '.' => vec![*direction],
            _ => panic!("Illegal character found"),
        }
    }
}

fn solve(
    position: (isize, isize),
    direction: Direction,
    grid: &mut Grid,
    move_set: &mut HashSet<((isize, isize), Direction)>,
) {
    if !move_set.insert((position, direction.clone())) {
        return;
    }

    let new_position = grid.get_next_position(&position, &direction);

    if let Some(new_position) = new_position {
        grid.covered[new_position.0 as usize][new_position.1 as usize] = true;
        let new_directions = grid.get_next_direction(&new_position, &direction);

        for new_direction in new_directions {
            solve(new_position, new_direction, grid, move_set);
        }
    }
}

fn solution(input_str: &str) -> String {
    let grid = Grid::new(
        input_str
            .lines()
            .map(|line| line.chars().map(|ch| ch).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );

    let mut initial_positions = Vec::new();
    for i in 0..grid.values.len() {
        initial_positions.push(((i as isize, -1), Direction::Right));
        initial_positions.push(((i as isize, grid.max_cols), Direction::Left));
    }

    for i in 0..grid.values[0].len() {
        initial_positions.push(((-1, i as isize), Direction::Down));
        initial_positions.push(((grid.max_rows, i as isize), Direction::Up));
    }

    let mut scores = Vec::new();
    for (current_position, initial_direction) in initial_positions {
        let mut move_set = HashSet::new();

        let mut grid_clone = grid.clone();
        solve(
            current_position,
            initial_direction,
            &mut grid_clone,
            &mut move_set,
        );

        scores.push(
            grid_clone
                .covered
                .iter()
                .map(|line| line.iter().map(|x| *x as isize).sum::<isize>())
                .sum::<isize>(),
        );
    }
    scores.iter().max().unwrap().to_string()
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
        assert_eq!(result, "51");
    }
}
