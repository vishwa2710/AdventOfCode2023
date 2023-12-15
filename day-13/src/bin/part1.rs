fn main() {
    let result = include_str!("input.txt");
    println!("{}", solution(result));
}

struct Tile {
    grid: Vec<Vec<String>>,
}

impl Tile {
    fn get_max_rows(&self) -> usize {
        self.grid.len()
    }

    fn get_max_cols(&self) -> usize {
        self.grid[0].len()
    }

    fn get_column(&self, index: usize) -> String {
        self.grid.iter().map(|x| x[index].clone()).collect()
    }

    fn get_row(&self, index: usize) -> String {
        self.grid[index].iter().map(|x| x.clone()).collect()
    }

    fn get_columns(&self) -> Vec<String> {
        let mut columns = vec![];
        for i in 0..self.get_max_cols() {
            columns.push(self.get_column(i));
        }
        columns
    }

    fn get_rows(&self) -> Vec<String> {
        let mut rows = vec![];
        for i in 0..self.get_max_rows() {
            rows.push(self.get_row(i));
        }
        rows
    }
}

fn get_reflected(values: &Vec<String>) -> Option<usize> {
    (1..(values.len())).find(|idx| {
        values[0..*idx]
            .iter()
            .rev()
            .zip(values[(*idx)..values.len()].iter())
            .all(|(x, y)| x == y)
    })
}

fn solution(input_str: &str) -> String {
    input_str
        .split("\n\n")
        .map(|x| Tile {
            grid: x
                .lines()
                .map(|y| y.chars().map(|z| z.to_string()).collect())
                .collect::<Vec<Vec<String>>>(),
        })
        .fold(0, |acc, tile| {
            // horizontal check
            if let Some(r) = get_reflected(&tile.get_rows()) {
                acc + (r * 100)
            } else {
                let reflected = get_reflected(&tile.get_columns()).expect("No reflection found");
                acc + reflected
            }
        })
        .to_string()
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
        assert_eq!(result, "405");
    }

    #[test]
    fn other_reflection() {
        let result = solution(
            "..####..#
#.####.##
#.####.##
..####..#
.######.#
#..##..##
.#.##.#.#
#....#.#.
#.####.#.",
        );
        assert_eq!(result, "200");
    }
}
