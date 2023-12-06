fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

fn solution(input_str: &str) -> String {
    let times = input_str
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let distances = input_str
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let combined = times
        .into_iter()
        .zip(distances.into_iter())
        .collect::<Vec<(u32, u32)>>();

    combined
        .iter()
        .map(|(time, distance)| {
            (1..*time)
                .filter(|&speed| (speed * (time - speed)) > *distance)
                .count()
        })
        .product::<usize>()
        .to_string()
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
        assert_eq!(result, "288");
    }
}
