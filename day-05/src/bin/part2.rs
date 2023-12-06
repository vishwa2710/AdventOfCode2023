use aoc::{get_map, Line, RangeMap, MAP_NAMES};
use std::collections::HashMap;

fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

fn get_number_from_map(map: &Vec<RangeMap>, target_line: &Line) -> Vec<Line> {
    if target_line.end < map[0].source.start || map[map.len() - 1].source.end < target_line.start {
        return vec![*target_line; 1];
    }

    let mut line_vecs = Vec::new();

    // add extra line that has a section before the first element in the map
    if target_line.start < map[0].source.start {
        line_vecs.push({
            Line {
                start: target_line.start,
                end: map[0].source.start,
            }
        });
    }

    for element in map.iter() {
        if !target_line.intersects(&element.source) {
            continue;
        }

        let intersecting_line = element.source.overlap(&target_line);

        let intersecting_line = Line {
            start: intersecting_line.start + element.diff,
            end: intersecting_line.end + element.diff,
        };

        line_vecs.push(intersecting_line);
    }

    // add extra line that has a section after the last element in the map
    if map[map.len() - 1].source.end < target_line.end {
        line_vecs.push(Line {
            start: map[map.len() - 1].source.end,
            end: target_line.end,
        });
    }

    line_vecs
}

fn solution(input_str: &str) -> String {
    let seed_data = input_str
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut map_map: HashMap<&str, Vec<RangeMap>> = HashMap::new();
    for map_name in MAP_NAMES {
        let map = get_map(map_name, input_str);
        map_map.insert(map_name, map);
    }

    let mut min_seed_location = i128::MAX;
    for i in (1..seed_data.len()).step_by(2) {
        let mut output_value = vec![
            Line {
                start: seed_data[i - 1],
                end: (seed_data[i - 1] + seed_data[i]),
            };
            1
        ];

        for map_name in MAP_NAMES {
            let map = &map_map[map_name];

            output_value = output_value
                .iter()
                .flat_map(|x| get_number_from_map(map, x))
                .collect::<Vec<Line>>();
        }

        min_seed_location =
            min_seed_location.min(output_value.iter().map(|x| x.start).min().unwrap());
    }

    min_seed_location.to_string()
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
        assert_eq!(result, "46");
    }
}
