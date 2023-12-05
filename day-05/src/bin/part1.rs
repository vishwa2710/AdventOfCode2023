fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

const MAP_NAMES: [&str; 7] = [
    "seed-to-soil map",
    "soil-to-fertilizer map",
    "fertilizer-to-water map",
    "water-to-light map",
    "light-to-temperature map",
    "temperature-to-humidity map",
    "humidity-to-location map",
];

fn get_map(key: &str, input_str: &str) -> Vec<(u64, u64, u64)> {
    let mut destination_to_source: Vec<(u64, u64, u64)> = Vec::new();
    let mut in_destination_to_source_map = false;
    for line in input_str.lines().skip(1) {
        if line.contains(key) {
            in_destination_to_source_map = true;
            continue;
        }
        if line == "" {
            in_destination_to_source_map = false;
            continue;
        }
        if in_destination_to_source_map {
            let (seed, soil, range) = {
                let mut iter = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());
                (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                )
            };
            destination_to_source.push((seed, soil, range));
        }
    }

    destination_to_source.sort_by_key(|k| k.1);
    destination_to_source
}

fn get_number_from_map(map: &Vec<(u64, u64, u64)>, key: &u64) -> u64 {
    for (destination_start, source_start, range) in map {
        if key < source_start {
            return *key;
        }
        if key >= source_start && key < &(source_start + range) {
            return destination_start + (key - source_start);
        }
    }
    *key
}

fn solution(input_str: &str) -> String {
    let seeds = input_str
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    seeds
        .iter()
        .map(|seed| {
            let mut output_value = *seed;
            for map_name in MAP_NAMES {
                let map = get_map(map_name, input_str);
                output_value = get_number_from_map(&map, &output_value);
            }
            output_value
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "35");
    }
}
