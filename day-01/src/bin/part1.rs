fn main() {
    let input_str = include_str!("input1.txt");
    let result = part1(input_str);
    println!("{}", result);
}

fn part1(input_str: &str) -> String {
    input_str
        .split("\n")
        .filter_map(|x| {
            let numeric_chars: String = x.chars().filter(|c| c.is_numeric()).collect();
            Some(
                format!(
                    "{}{}",
                    numeric_chars.chars().nth(0).unwrap(),
                    numeric_chars.chars().nth(numeric_chars.len() - 1).unwrap()
                )
                .parse::<u32>()
                .expect("This is not a valid u32"),
            )
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }
}
