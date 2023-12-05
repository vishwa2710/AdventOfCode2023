use std::collections::HashMap;

fn main() {
    let input_str = include_str!("input1.txt");
    let result = part1(input_str);
    println!("{}", result);
}

fn part1(input_str: &str) -> String {
    let games = input_str.split("\n").collect::<Vec<&str>>();

    let mut score = 0;

    let mut limits: HashMap<&str, u32> = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);

    // game = Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    for game in games {
        // tmp = 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let tmp = game.split(":").collect::<Vec<&str>>();
        let game_id = tmp[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();

        // rounds = ["3 blue, 4 red", "1 red, 2 green, 6 blue", 2 green]
        let rounds = tmp[1].split(";").collect::<Vec<&str>>();

        let mut game_is_valid = true;

        // round = "3 blue, 4 red"
        for round in rounds {
            // plays = ["3 blue", "4 red"]
            let plays = round.split(",").map(|x| x.trim()).collect::<Vec<&str>>();

            // play = "3 blue"
            for play in plays {
                let tmp = play.split(" ").collect::<Vec<&str>>();
                let count = tmp[0].trim().parse::<u32>().unwrap();
                let color = tmp[1].trim();
                if count > limits[color] {
                    game_is_valid = false;
                    break;
                }
            }
            if !game_is_valid {
                break;
            }
        }
        if game_is_valid {
            score += game_id;
        }
    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }
}
