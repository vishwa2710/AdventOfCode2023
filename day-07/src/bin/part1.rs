use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("{}", result);
}

#[derive(Debug, Hash)]
struct Card {
    value: char,
}

impl Card {
    fn score(&self) -> u64 {
        match self.value {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'J' => 3,
            'T' => 4,
            '9' => 5,
            '8' => 6,
            '7' => 7,
            '6' => 8,
            '5' => 9,
            '4' => 10,
            '3' => 11,
            '2' => 12,
            _ => panic!("Invalid card"),
        }
    }
}

struct Hand {
    cards: [Card; 5],
    bid: u64,
    rank: u64,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: u64) -> Hand {
        let mut rank = 0;

        let mut counts = HashMap::new();

        for card in cards.iter() {
            counts
                .entry(card.value)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let create_rank = |value: u64| {
            format!(
                "{}{:02}{:02}{:02}{:02}{:02}",
                value,
                cards[0].score(),
                cards[1].score(),
                cards[2].score(),
                cards[3].score(),
                cards[4].score(),
            )
            .parse::<u64>()
            .unwrap()
        };

        // 5 of a kind
        if counts.len() == 1 {
            rank = create_rank(1);
        // 4 of a kind or Full house
        } else if counts.len() == 2 {
            for (_, count) in counts.iter() {
                if *count == 4 {
                    rank = create_rank(2);
                    break;
                } else if *count == 3 {
                    rank = create_rank(3);
                    break;
                }
            }
        // 3 of a kind or 2 pair
        } else if counts.len() == 3 {
            for (_, count) in counts.iter() {
                if *count == 3 {
                    rank = create_rank(4);
                    break;
                } else if *count == 2 {
                    rank = create_rank(5);
                    break;
                }
            }
        // 2 of a kind
        } else if counts.len() == 4 {
            rank = create_rank(6);
        } else {
            rank = create_rank(7);
        }

        Hand { cards, bid, rank }
    }
}

fn solution(input_str: &str) -> String {
    let mut tmp = input_str
        .lines()
        .map(|line| {
            let data = line.split_whitespace().collect::<Vec<&str>>();
            let hand = Hand::new(
                [
                    Card {
                        value: data[0].chars().nth(0).unwrap(),
                    },
                    Card {
                        value: data[0].chars().nth(1).unwrap(),
                    },
                    Card {
                        value: data[0].chars().nth(2).unwrap(),
                    },
                    Card {
                        value: data[0].chars().nth(3).unwrap(),
                    },
                    Card {
                        value: data[0].chars().nth(4).unwrap(),
                    },
                ],
                data[1].parse::<u64>().unwrap(),
            );
            hand
        })
        .collect::<Vec<Hand>>();
    tmp.sort();
    tmp.iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "6440");
    }
}
