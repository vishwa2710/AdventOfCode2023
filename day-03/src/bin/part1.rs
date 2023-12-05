use std::collections::HashSet;

fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

#[derive(Debug)]
struct PartNumber {
    number: u32,
    indices: Vec<(usize, usize)>,
}

impl PartNumber {
    fn get_adjacent_indices(&self, row_max: &usize, col_max: &usize) -> HashSet<(usize, usize)> {
        let mut adjacent_indices: HashSet<(usize, usize)> = HashSet::new();
        for (row, col) in self.indices.iter() {
            // why do I have to borrow from an int? O_o
            if row > &0 {
                adjacent_indices.insert((row - 1, *col));
            }
            if row < &(row_max - 1) {
                adjacent_indices.insert((row + 1, *col));
            }
            if col > &0 {
                adjacent_indices.insert((*row, col - 1));
            }
            if col < &(col_max - 1) {
                adjacent_indices.insert((*row, col + 1));
            }
            if row < &(row_max - 1) && col < &(col_max - 1) {
                adjacent_indices.insert((row + 1, col + 1));
            }
            if row > &0 && col > &0 {
                adjacent_indices.insert((row - 1, col - 1));
            }
            if row < &(row_max - 1) && col > &0 {
                adjacent_indices.insert((row + 1, col - 1));
            }
            if row > &0 && col < &(col_max - 1) {
                adjacent_indices.insert((row - 1, col + 1));
            }
        }
        adjacent_indices
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && (c != '.')
}

fn is_part_number(indices: &HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) -> bool {
    for (row, col) in indices.iter() {
        if is_symbol(grid[*row][*col]) {
            return true;
        }
    }
    false
}

fn solution(input_str: &str) -> String {
    let lines: Vec<&str> = input_str.split("\n").collect();
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let row_max = lines.len();
    grid.resize(row_max, Vec::new());
    for (row, line) in lines.iter().enumerate() {
        grid[row] = line.chars().collect();

        let mut is_digit = false;
        let mut digit = String::new();
        let mut indices = Vec::new();
        for (index, c) in line.char_indices() {
            if c.is_digit(10) {
                is_digit = true;
                digit.push(c);
                indices.push((row, index));
            } else {
                is_digit = false;
            }

            if !is_digit && !digit.is_empty() {
                part_numbers.push(PartNumber {
                    number: digit.parse::<u32>().unwrap(),
                    indices: indices.clone(),
                });
                digit = String::new();
                indices = Vec::new();
            }
        }
        if is_digit {
            part_numbers.push(PartNumber {
                number: digit.parse::<u32>().unwrap(),
                indices: indices.clone(),
            });
        }
    }

    let col_max = grid[0].len();

    part_numbers
        .iter()
        .map(|part_number| {
            if is_part_number(&part_number.get_adjacent_indices(&row_max, &col_max), &grid) {
                println!(" {:?}", part_number.number);
                part_number.number
            } else {
                0
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
        let result = solution(
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
        assert_eq!(result, "4361");
    }
}
