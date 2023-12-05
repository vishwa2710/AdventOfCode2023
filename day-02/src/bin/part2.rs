use std::collections::HashMap;

fn main() {
    let input_str = include_str!("input1.txt");
    let result = part2(input_str);
    println!("{}", result);
}

fn part2(input_str: &str) -> String {
    let games = input_str.split("\n").collect::<Vec<&str>>();

    let mut score = 0;

    // game = Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    for game in games {
        // tmp = 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let sets = game.split(":").collect::<Vec<&str>>()[1];

        // rounds = ["3 blue, 4 red", "1 red, 2 green, 6 blue", 2 green]
        let rounds = sets.split(";").collect::<Vec<&str>>();

        // round = "3 blue, 4 red"
        let mut colors_count = HashMap::new();
        colors_count.insert("red", vec![]);
        colors_count.insert("green", vec![]);
        colors_count.insert("blue", vec![]);

        for round in rounds {
            // play = "3 blue"
            for play in round.split(",") {
                let tmp = play.trim().split(" ").collect::<Vec<&str>>();
                let count = tmp[0].trim().parse::<u32>().expect("cannot parse to u32");
                let color = tmp[1].trim();
                colors_count
                    .get_mut(color)
                    .expect("color missing")
                    .push(count);
            }
        }

        score += colors_count
            .iter()
            .map(|(_, v)| v.iter().max().expect("empty vector"))
            .product::<u32>();
    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286");
    }
}
