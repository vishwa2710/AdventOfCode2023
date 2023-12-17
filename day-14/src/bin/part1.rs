fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("{result}");
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

fn solution(input_str: &str) -> String {
    let grid = input_str
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let grid = apply_north_tilt(&grid);

    grid.iter()
        .zip((1..grid.len() + 1).rev())
        .map(|(row, idx)| row.iter().filter(|ch| **ch == 'O').count() * idx)
        .sum::<usize>()
        .to_string()
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
        assert_eq!(result, "136");
    }
}
