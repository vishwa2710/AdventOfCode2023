use aoc::roots;

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
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let distances = input_str
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let combined = times
        .into_iter()
        .zip(distances.into_iter())
        .collect::<Vec<(i64, i64)>>();

    combined
        .iter()
        .map(|(time, distance)| {
            let (min_bound, max_bound) = roots(1, -*time, *distance).unwrap();
            // since we want to "further", we want to take the ceil of the min bound and the floor of the max bound
            // and then subtract 1 since we want the number of integers between the two bounds
            (max_bound.ceil() - min_bound.floor()).abs() as usize - 1
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
