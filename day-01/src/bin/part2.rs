fn main() {
    let input_str = include_str!("input2.txt");
    let result = part2(input_str);
    println!("{}", result);
}

const PATTERN: &[(&'static str, &'static str)] = &[
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn replace_substring(input_str: &str) -> String {
    let mut input_str: String = input_str.to_string();
    for (from, to) in PATTERN {
        input_str = input_str.replace(from, to);
    }
    input_str.to_string()
}

fn part2(input_str: &str) -> String {
    input_str
        .split("\n")
        .map(replace_substring)
        .map(|x| {
            let numeric_chars: String = x.chars().filter(|c| c.is_numeric()).collect();
            format!(
                "{}{}",
                numeric_chars.chars().nth(0).unwrap(),
                numeric_chars.chars().nth(numeric_chars.len() - 1).unwrap()
            )
            .parse::<u32>()
            .expect("This is not a valid u32")
        })
        .reduce(|a, b| a + b)
        .expect("no result found")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
