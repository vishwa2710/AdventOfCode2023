fn main() {
    let result = include_str!("input.txt");
    println!("{}", solution(result));
}

fn is_valid_combination(reading: &str, groups: &Vec<usize>) -> bool {
    let parsed_reading = reading.split(|c| c != '#').filter(|s| !s.is_empty());
    if parsed_reading.clone().count() != groups.len() {
        false
    } else {
        parsed_reading
            .zip(groups.iter())
            .all(|(r, g)| r.len() == *g)
    }
}

fn generate_combinations(s: &str, n: usize, groups: &Vec<usize>) -> Vec<String> {
    if n == 0 {
        if is_valid_combination(s, groups) {
            return vec![s.to_string()];
        } else {
            return vec![];
        }
    }

    let mut combinations = Vec::new();

    for c in &['.', '#'] {
        let replaced = s.replacen("?", &c.to_string(), 1);
        combinations.extend(generate_combinations(&replaced, n - 1, groups));
    }

    combinations
}

fn solution(input_str: &str) -> String {
    input_str
        .lines()
        .map(|line| {
            let readings = std::iter::repeat(line.split_whitespace().nth(0).unwrap())
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let groups = std::iter::repeat(line.split_whitespace().nth(1).unwrap())
                .take(5)
                .collect::<Vec<_>>()
                .join(",")
                .split(",")
                .map(|group| group.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let combinations =
                generate_combinations(readings.as_str(), readings.matches('?').count(), &groups)
                    .iter()
                    .filter(|c| is_valid_combination(c.as_str(), &groups))
                    .count();

            combinations
        })
        .sum::<usize>()
        .to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, "525152");
    }
}
