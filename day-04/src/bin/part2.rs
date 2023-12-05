use std::collections::HashSet;

fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

fn solution(input_str: &str) -> String {
    let mut card_vec = vec![0; input_str.split("\n").count()];
    input_str
        .split("\n")
        .map(|line| line.split(":").nth(1).unwrap())
        .map(|line| line.split("|").map(|x| x.trim()).collect::<Vec<&str>>())
        .map(|line| {
            line.iter()
                .map(|y| {
                    y.split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .collect::<Vec<HashSet<u32>>>()
        })
        .enumerate()
        .for_each(|(card_number, card)| {
            let intersection_count = card[0].intersection(&card[1]).count();
            card_vec[card_number] += 1;
            for _ in 0..card_vec[card_number] {
                for i in 0..intersection_count {
                    let index = card_number + i + 1;
                    if index < card_vec.len() {
                        card_vec[index] += 1;
                    }
                }
            }
        });

    println!("{:?}", card_vec);
    card_vec.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30");
    }
}
