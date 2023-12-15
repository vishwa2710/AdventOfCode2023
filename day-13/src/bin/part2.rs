use std::fmt;

fn main() {
    let result = include_str!("input.txt");
    println!("{}", solution(result));
}

#[derive(Clone)]
struct Tile {
    grid: Vec<Vec<char>>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        for i in &self.grid {
            for j in i {
                string.push(*j);
            }
            string.push('\n');
        }
        write!(f, "{}", string)
    }
}

impl Tile {
    fn get_max_rows(&self) -> usize {
        self.grid.len()
    }

    fn get_max_cols(&self) -> usize {
        self.grid[0].len()
    }

    fn transpose(&self) -> Tile {
        let mut transposed_grid: Vec<Vec<char>> = Vec::new();
        for i in 0..self.get_max_cols() {
            let mut col = Vec::new();
            for j in 0..self.get_max_rows() {
                col.push(self.grid[j][i]);
            }
            transposed_grid.push(col);
        }

        Tile {
            grid: transposed_grid,
        }
    }
}

fn get_column(tile: &Tile, index: usize) -> String {
    tile.grid.iter().map(|x| x[index].clone()).collect()
}

fn get_row(tile: &Tile, index: usize) -> String {
    tile.grid[index].iter().map(|x| x.clone()).collect()
}

fn get_columns(tile: &Tile) -> Vec<String> {
    let mut columns = vec![];
    for i in 0..tile.get_max_cols() {
        columns.push(get_column(tile, i));
    }
    columns
}

fn get_rows(tile: &Tile) -> Vec<String> {
    let mut rows = vec![];
    for i in 0..tile.get_max_rows() {
        rows.push(get_row(tile, i));
    }
    rows
}

fn get_reflected(tile: &Tile) -> Vec<usize> {
    let get_idxs = |values: &Vec<String>| {
        let mut idxs = Vec::new();
        for idx in 0..values.len() {
            // println!("idx: {idx}");
            let first_iter = values[0..idx].iter().rev();
            let second_iter = values[idx..values.len()].iter();

            let mut similar_values = Vec::new();
            for (v1, v2) in first_iter.zip(second_iter) {
                // println!("{v1} == {v2}, {}", v1 == v2);
                similar_values.push(v1 == v2);
            }
            if !similar_values.is_empty() && similar_values.iter().all(|x| *x) {
                idxs.push(idx);
            }
        }

        return idxs;
    };

    let row_idxs = get_idxs(&get_rows(tile));

    let col_idxs = get_idxs(&get_rows(&tile.transpose()));

    let mut scores = Vec::new();
    for row_idx in row_idxs {
        scores.push(row_idx * 100);
    }
    for col_idx in col_idxs {
        scores.push(col_idx);
    }

    scores
}

fn get_scores(tile: &Tile) -> usize {
    let original_scores = get_reflected(tile);
    if original_scores.len() > 1 {
        panic!("Extra scores");
    }
    let original_score = original_scores[0];

    for i in 0..tile.get_max_rows() {
        for j in 0..tile.get_max_cols() {
            let mut tile_clone = tile.clone();
            match tile_clone.grid[i][j] {
                '#' => tile_clone.grid[i][j] = '.',
                '.' => tile_clone.grid[i][j] = '#',
                _ => panic!("Illegal character"),
            };

            let scores = get_reflected(&tile_clone);
            if !&scores.is_empty() {
                if (scores.len() == 1) && (scores[0] == original_score) {
                    continue;
                }
                let score = scores
                    .iter()
                    .find(|x| **x != original_score)
                    .expect("No solution found.");

                return *score;
            }
        }
    }

    panic!(
        "No solution found for tile:\n{}\ntransposed:\n{}",
        tile,
        tile.transpose()
    );
}

fn solution(input_str: &str) -> String {
    let tiles: Vec<_> = input_str
        .split("\n\n")
        .map(|x| Tile {
            grid: x
                .lines()
                .map(|y| y.chars().map(|z: char| z).collect())
                .collect::<Vec<Vec<char>>>(),
        })
        .collect();

    let mut score = 0;
    for tile in tiles.iter() {
        score += get_scores(tile);
    }

    score.to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, "400");
    }
}
