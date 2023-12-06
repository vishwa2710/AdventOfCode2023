use aoc::roots;

fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

fn solution(input_str: &str) -> String {
    let time: i64 = input_str
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .flat_map(|x| x.chars())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distance: i64 = input_str
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .flat_map(|x| x.chars())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let (min_root, max_root) = roots(1, -time, distance as i64).unwrap();

    ((max_root.ceil() - min_root.floor()).abs() as usize - 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "71503");
    }
}
