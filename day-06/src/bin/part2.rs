fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

fn solution(input_str: &str) -> String {
    let time: u128 = input_str
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .flat_map(|x| x.chars())
        .collect::<String>()
        .parse::<u128>()
        .unwrap();
    let distance: u128 = input_str
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .flat_map(|x| x.chars())
        .collect::<String>()
        .parse::<u128>()
        .unwrap();

    let mut counter = 0;
    for hold_time in 1..time {
        let speed = hold_time;
        let distance_covered = speed * (time - hold_time);
        if distance_covered > distance {
            counter += 1;
        }
    }

    counter.to_string()
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
