use std::collections::HashMap;

fn main() {
    println!("{}", solution(include_str!("input.txt")).to_string());
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

fn solution(input_str: &str) -> String {
    let directions = input_str.lines().nth(0).unwrap();

    let tmp = input_str.lines().skip(2).map(|line| {
        let data = line.split("=").collect::<Vec<&str>>();
        let origin = data[0].trim();
        let destination = data[1]
            .trim()
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(",")
            .collect::<Vec<&str>>();
        let mut middle_map = HashMap::new();
        middle_map.insert('L', destination[0].trim());
        middle_map.insert('R', destination[1].trim());
        (origin, middle_map)
    });
    let graph: HashMap<&str, HashMap<char, &str>> = HashMap::from_iter(tmp);

    let origins: Vec<&str> = graph
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|x| *x)
        .collect();

    let mut directions_cycle: std::iter::Cycle<std::str::Chars<'_>> = directions.chars().cycle();

    let mut node_counters: Vec<u128> = vec![0; origins.len()];
    for (index, origin) in origins.iter().enumerate() {
        let mut origin = origin;
        while !origin.ends_with("Z") {
            origin = graph
                .get(origin)
                .unwrap()
                .get(&directions_cycle.next().unwrap())
                .unwrap();
            node_counters[index] += 1;
        }
    }

    let mut lcm_value = node_counters[0];

    for &num in &node_counters[1..] {
        lcm_value = lcm(lcm_value, num);
    }

    lcm_value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6");
    }
}
