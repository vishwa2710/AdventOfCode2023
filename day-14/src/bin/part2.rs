use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("{result}");
}

fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            new_grid[j][rows - 1 - i] = grid[i][j];
        }
    }

    new_grid
}

fn apply_north_tilt(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut prev_grid = grid.clone();
    let mut grid = grid.clone();

    loop {
        for i in 1..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'O' {
                    match grid[i - 1][j] {
                        '.' => {
                            grid[i - 1][j] = 'O';
                            grid[i][j] = '.';
                        }
                        _ => continue,
                    }
                }
            }
        }

        if prev_grid == grid {
            break;
        } else {
            prev_grid = grid.clone();
        }
    }

    grid
}

fn apply_cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = grid.clone();
    for _ in 0..4 {
        grid = apply_north_tilt(&grid);
        grid = rotate_grid(&grid);
    }
    grid
}

fn get_score(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .zip((1..grid.len() + 1).rev())
        .map(|(row, idx)| row.iter().filter(|ch| **ch == 'O').count() * idx)
        .sum::<usize>()
}

fn solution(input_str: &str) -> String {
    let mut grid = input_str
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut computed_grids = HashSet::new();
    let mut grid_order = Vec::new();
    let target_grid;
    loop {
        if !computed_grids.insert(grid.clone()) {
            target_grid = grid.clone();
            break;
        }

        grid_order.push(grid.clone());
        grid = apply_cycle(&grid);
    }

    let start = grid_order
        .iter()
        .position(|g| *g == target_grid)
        .expect("Grid not found");

    let idx = ((1000000000 - start) % (grid_order.len() - start)) + start;
    get_score(&grid_order[idx]).to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, "64");
    }
}
