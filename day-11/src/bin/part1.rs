use std::collections::HashSet;

fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

fn solution(input_str: &str) -> String {
    let galaxies_index = input_str
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, line)| {
            acc.extend(
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .map(|(col, _)| (row, col)),
            );
            acc
        });

    let row_max = input_str.lines().count();
    let col_max = input_str.lines().nth(0).unwrap().chars().count();

    let rows_with_galaxies = galaxies_index
        .iter()
        .map(|(row, _)| *row)
        .collect::<HashSet<usize>>();
    let cols_with_galaxies = galaxies_index
        .iter()
        .map(|(_, col)| *col)
        .collect::<HashSet<usize>>();

    let rows_without_galaxies = (0..row_max)
        .collect::<HashSet<usize>>()
        .difference(&rows_with_galaxies)
        .map(|x| *x)
        .collect::<HashSet<usize>>();

    let cols_without_galaxies = (0..col_max)
        .collect::<HashSet<usize>>()
        .difference(&cols_with_galaxies)
        .map(|x| *x)
        .collect::<HashSet<usize>>();

    let mut galaxies_distances = 0;

    for i in 0..galaxies_index.len() {
        for j in i + 1..galaxies_index.len() {
            let (row, col) = galaxies_index[i];
            let (row2, col2) = galaxies_index[j];

            let (col, col2) = {
                if col > col2 {
                    (col2, col)
                } else {
                    (col, col2)
                }
            };

            let (row, row2) = {
                if row > row2 {
                    (row2, row)
                } else {
                    (row, row2)
                }
            };

            let row_distance = row.abs_diff(row2);
            let col_distance = col.abs_diff(col2);

            let buffer_rows = rows_without_galaxies
                .iter()
                .filter(|x| **x > row && **x < row2)
                .count();

            let buffer_cols = cols_without_galaxies
                .iter()
                .filter(|x| **x > col && **x < col2)
                .count();

            galaxies_distances += row_distance + col_distance + buffer_rows + buffer_cols;
        }
    }

    galaxies_distances.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, "374");
    }
}
