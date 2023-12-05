use std::collections::{HashMap, HashSet};

fn main() {
    let input_str = include_str!("input.txt");
    let result = find_gear_product(input_str);
    println!("{}", result);
}

#[derive(Debug, Clone)]
struct PartNumber {
    number: u32,
    indices: Vec<(usize, usize)>,
}

impl PartNumber {
    fn get_adjacent_indices(&self, row_max: usize, col_max: usize) -> HashSet<(usize, usize)> {
        let mut adjacent_indices = HashSet::new();
        for &(row, col) in &self.indices {
            for &(dr, dc) in &[(1, 0), (0, 1), (1, 1), (-1, -1), (1, -1), (-1, 1)] {
                let (adj_row, adj_col) = (row as isize + dr, col as isize + dc);
                if adj_row >= 0
                    && adj_row < row_max as isize
                    && adj_col >= 0
                    && adj_col < col_max as isize
                {
                    adjacent_indices.insert((adj_row as usize, adj_col as usize));
                }
            }
        }
        adjacent_indices
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn find_gear_product(input_str: &str) -> String {
    let lines: Vec<&str> = input_str.split('\n').collect();

    let row_max = lines.len();
    let col_max = lines.first().map_or(0, |line| line.len());

    let mut part_numbers = Vec::new();
    let grid: Vec<Vec<char>> = lines.iter().map(|&line| line.chars().collect()).collect();

    for (row, line) in lines.iter().enumerate() {
        let mut digit = String::new();
        let mut indices = Vec::new();

        for (col, c) in line.char_indices() {
            if c.is_digit(10) {
                digit.push(c);
                indices.push((row, col));
            } else if !digit.is_empty() {
                part_numbers.push(PartNumber {
                    number: digit.parse::<u32>().unwrap(),
                    indices,
                });
                digit.clear();
                indices = Vec::new();
            }
        }

        if !digit.is_empty() {
            part_numbers.push(PartNumber {
                number: digit.parse::<u32>().unwrap(),
                indices,
            });
        }
    }

    let mut gear_count: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for part_number in part_numbers {
        if part_number
            .get_adjacent_indices(row_max, col_max)
            .iter()
            .any(|&(r, c)| is_symbol(grid[r][c]))
        {
            for &(row, col) in &part_number.get_adjacent_indices(row_max, col_max) {
                if grid[row][col] == '*' {
                    gear_count
                        .entry((row, col))
                        .or_insert_with(Vec::new)
                        .push(part_number.number);
                }
            }
        }
    }

    gear_count
        .values()
        .filter_map(|values| {
            if values.len() == 2 {
                Some(values[0] * values[1])
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = find_gear_product(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835");
    }
}
