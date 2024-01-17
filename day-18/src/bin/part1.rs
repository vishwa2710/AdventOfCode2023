use std::collections::HashSet;

use geo::{coord, Area, BooleanOps, Coord, GeodesicArea, LineString, Polygon};

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
    fn from_char(ch: char) -> Direction {
        match ch {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

fn take_step(
    current_position: &(isize, isize),
    direction: &Direction,
    step_size: &isize,
) -> (isize, isize) {
    match direction {
        Direction::Up => (current_position.0 - step_size, current_position.1),
        Direction::Down => (current_position.0 + step_size, current_position.1),
        Direction::Left => (current_position.0, current_position.1 - step_size),
        Direction::Right => (current_position.0, current_position.1 + step_size),
    }
}

fn get_steps(
    current_position: &(isize, isize),
    direction: &Direction,
    step_size: &u32,
) -> Vec<(isize, isize)> {
    match direction {
        Direction::Up => (1..=(*step_size + 1 as u32))
            .map(|s| (current_position.0 - s as isize, current_position.1))
            .collect::<Vec<(isize, isize)>>(),
        Direction::Down => (1..=(*step_size + 1 as u32))
            .map(|s| (current_position.0 + s as isize, current_position.1))
            .collect::<Vec<(isize, isize)>>(),
        Direction::Left => (1..=(*step_size + 1 as u32))
            .map(|s| (current_position.0, current_position.1 - s as isize))
            .collect::<Vec<(isize, isize)>>(),
        Direction::Right => (1..=(*step_size + 1 as u32))
            .map(|s| (current_position.0, current_position.1 + s as isize))
            .collect::<Vec<(isize, isize)>>(),
    }
}

fn solution(input_str: &str) -> String {
    let mut coords: Vec<Coord> = vec![coord![x: 0f64, y: 0f64]];

    let mut horizontal_edges = HashSet::new();

    input_str.lines().enumerate().for_each(|(idx, line)| {
        let mut split_line = line.split(" ");

        let direction = Direction::from_char(split_line.next().unwrap().chars().nth(0).unwrap());
        let step_size = split_line.next().unwrap().parse::<isize>().unwrap();
        let position = (coords[idx].x as isize, coords[idx].y as isize);
        let next_position = take_step(&position, &direction, &step_size);

        coords.push(coord![x: next_position.0 as f64, y: next_position.1 as f64]);

        println!("{position:?} + {direction:?} => next position: {next_position:?}");

        if direction == Direction::Left || direction == Direction::Right {
            horizontal_edges.insert(next_position.0);
        }
    });

    let mut horizontal_edges = horizontal_edges.iter().map(|x| *x).collect::<Vec<isize>>();
    horizontal_edges.sort();
    println!("HORIZONTAL EDGES: {horizontal_edges:?}");

    let min_y = coords
        .iter()
        .map(|coord| coord.y as isize)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let max_y = coords
        .iter()
        .map(|coord| coord.y as isize)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("MIN X: {min_y}, MAX_X: {max_y}");

    let linestring = LineString::new(coords);
    let polygon = Polygon::new(linestring.clone(), vec![]);

    println!("POLYGON: {polygon:?}");

    let mut score = 0;
    for x in horizontal_edges.windows(2) {
        let bounding_box = Polygon::new(
            LineString::new(vec![
                coord![x: x[0] as f64, y: min_y as f64],
                coord![x: x[0] as f64, y: max_y as f64],
                coord![x: x[1] as f64, y: max_y as f64],
                coord![x: x[1] as f64, y: min_y as f64],
                coord![x: x[0] as f64, y: min_y as f64],
            ]),
            vec![],
        );
        let new_box = polygon.intersection(&bounding_box);
        println!("bounding_box: {bounding_box:?}");
        for (idx, thing) in new_box.iter().enumerate() {
            println!("{idx}: {:?} => {}", thing, thing.signed_area());
            score += thing.signed_area() as i32;
        }
    }

    println!("SCORE: {score}");
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

    // input_str.lines().for_each(|line| {
    //     let mut split_line = line.split(" ");

    //     let direction = Direction::from_char(split_line.next().unwrap().chars().nth(0).unwrap());
    //     let step_size = split_line.next().unwrap().parse::<u32>().unwrap();

    //     let next_positions = get_steps(&current_position, &direction, &step_size);
    //     println!("{current_position:?} + {direction:?} + {step_size:?} = {next_positions:?}");
    //     current_position = *next_positions.last().unwrap();

    //     next_positions.iter().for_each(|next_position| {
    //         coords
    //             .entry(next_position.0 as usize)
    //             .or_insert(HashSet::new())
    //             .insert(next_position.1 as usize);
    //     });
    // });

    println!("coords: {polygon:?}");
    "5".to_string()
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
