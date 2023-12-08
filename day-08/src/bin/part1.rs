use std::collections::HashMap;

fn main() {
    println!("{}", solution(include_str!("input.txt")).to_string());
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
    let map: HashMap<&str, HashMap<char, &str>> = HashMap::from_iter(tmp);

    let mut counter: i128 = 0;
    let mut output = "AAA";
    let mut directions_cycle = directions.chars().cycle();
    while output != "ZZZ" {
        let direction = directions_cycle.next().unwrap();
        output = map.get(output).unwrap().get(&direction).unwrap();
        counter += 1
    }
    if counter % 1e7 as i128 == 0 {
        println!("Counter: {}", counter);
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6");
    }
}
