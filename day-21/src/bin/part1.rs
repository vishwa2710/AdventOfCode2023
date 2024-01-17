use std::collections::HashSet;

fn main() {
    let result = solution(include_str!("input.txt"));
    println!("{}", result);
}

fn get_steps(coordinate: &(usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut steps = vec![];

    let (x, y) = coordinate;

    if y > &0 && grid[*x][y - 1] != '#' {
        steps.push((*x, y - 1));
    }

    if y < &(grid.len() - 1) && grid[*x][y + 1] != '#' {
        steps.push((*x, y + 1));
    }

    if x > &0 && grid[x - 1][*y] != '#' {
        steps.push((x - 1, *y));
    }

    if x < &(grid[0].len() - 1) && grid[x + 1][*y] != '#' {
        steps.push((x + 1, *y));
    }

    steps
}

fn solution(input_str: &str) -> String {
    let grid = input_str
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_coordinate = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    let mut steps_remaining = 64;

    let mut current_coordinates = HashSet::new();
    current_coordinates.insert(start_coordinate);

    while steps_remaining > 0 {
        let mut next_coordinates = HashSet::new();
        for coordinate in current_coordinates.iter() {
            next_coordinates.extend(get_steps(coordinate, &grid));
        }

        current_coordinates = next_coordinates;
        steps_remaining -= 1;
        println!(
            "{} steps remaining: {} coordinates",
            steps_remaining,
            current_coordinates.len()
        );
    }

    current_coordinates.len().to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        );
        assert_eq!(result, "16");
    }
}
