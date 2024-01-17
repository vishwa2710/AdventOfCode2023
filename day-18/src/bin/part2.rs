use std::collections::HashMap;

fn main() {
    let result = solution(include_str!("input.txt"));
    println!("{}", result);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_int(ch: &u32) -> Direction {
        match ch {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }
}

fn get_steps(
    current_position: &(isize, isize),
    direction: &Direction,
    step_size: &u32,
) -> Vec<(isize, isize)> {
    match direction {
        Direction::Up => (0..(*step_size as u32))
            .map(|_| (current_position.0 - *step_size as isize, current_position.1))
            .collect::<Vec<(isize, isize)>>(),
        Direction::Down => (0..(*step_size as u32))
            .map(|_| (current_position.0 + *step_size as isize, current_position.1))
            .collect::<Vec<(isize, isize)>>(),
        Direction::Left => vec![(current_position.0, current_position.1 - *step_size as isize)],
        Direction::Right => vec![(current_position.0, current_position.1 + *step_size as isize)],
    }
}

fn solution(input_str: &str) -> String {
    let mut current_position = (0, 0);
    let mut coords: HashMap<isize, Vec<isize>> = HashMap::new();

    input_str.lines().enumerate().for_each(|(idx, line)| {
        let split_line = line.split(" ").nth(2).unwrap();

        let step_size = u32::from_str_radix(&split_line[2..7], 16).unwrap();
        let direction =
            Direction::from_int(&split_line.chars().nth(7).unwrap().to_digit(10).unwrap());

        let next_positions = get_steps(&current_position, &direction, &step_size);
        current_position = *next_positions.last().unwrap();

        next_positions.iter().for_each(|next_position| {
            coords
                .entry(next_position.0)
                .or_insert(vec![])
                .push(next_position.1);
        });
    });

    for (_, value) in coords.iter_mut() {
        value.sort();
        if (value.len() % 2) != 0 {
            println!("{:?}", value);
        }
    }

    // println!("{:?}", coords);

    // let min_x = coords
    //     .iter()
    //     .map(|coord| coord.x as isize)
    //     .min_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // let max_x = coords
    //     .iter()
    //     .map(|coord| coord.x as isize)
    //     .max_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // let min_y = coords
    //     .iter()
    //     .map(|coord| coord.y as isize)
    //     .min_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // let max_y = coords
    //     .iter()
    //     .map(|coord| coord.y as isize)
    //     .max_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // let linestring = LineString::new(coords);
    // let polygon = Polygon::new(linestring.clone(), vec![]);

    // for i in min_x..max_x + 1 {
    //     for j in min_y..max_y + 1 {
    //         let point = coord![x: i as f64, y: j as f64];
    //         if polygon.intersects(&point) {
    //             // print!("#");
    //             points_covered += 1;
    //         } else {
    //             // print!(".");
    //         }
    //     }
    //     // println!("");
    // }

    // println!("{:?}", linestring);

    "12".to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        assert_eq!(result, "62");
    }
}
