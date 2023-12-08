use phf::{phf_map, Map};
use std::collections::HashMap;

const SCORES: Map<char, u8> = phf_map! {
    'A' => 1,
    'K' => 2,
    'Q' => 3,
    'J' => 13,
    'T' => 4,
    '9' => 5,
    '8' => 6,
    '7' => 7,
    '6' => 8,
    '5' => 9,
    '4' => 10,
    '3' => 11,
    '2' => 12,
};

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("{}", result);
}

#[derive(Debug, Hash, Clone, Copy)]
struct Card {
    value: char,
    score: u8,
}

impl Card {
    fn new(value: char) -> Card {
        Card {
            value,
            score: *SCORES.get(&value).unwrap(),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    rank: u64,
}

impl Hand {
    fn new(cards: &[Card; 5], bid: u64) -> Hand {
        let rank = get_rank(cards);
        Hand {
            cards: *cards,
            bid,
            rank,
        }
    }
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank == other.rank {
            for (index, card) in self.cards.iter().enumerate() {
                if card != &other.cards[index] {
                    return Some(card.cmp(&other.cards[index]));
                }
            }
        }
        Some(self.rank.cmp(&other.rank))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.rank == other.rank {
            for (index, card) in self.cards.iter().enumerate() {
                if card != &other.cards[index] {
                    return card.cmp(&other.cards[index]);
                }
            }
        }
        self.rank.cmp(&other.rank)
    }
}

fn get_count_map(cards: &[Card; 5]) -> HashMap<char, u64> {
    let mut counts = HashMap::new();

    for card in cards.iter() {
        counts
            .entry(card.value)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    counts
}

fn get_rank(cards: &[Card; 5]) -> u64 {
    let mut rank = 0;

    let counts = get_count_map(cards);

    // 5 of a kind
    if counts.len() == 1 {
        rank = 1;
    // 4 of a kind or Full house
    } else if counts.len() == 2 {
        for (_, count) in counts.iter() {
            if *count == 4 {
                rank = 2;
                break;
            } else if *count == 3 {
                rank = 3;
                break;
            }
        }
    // 3 of a kind or 2 pair
    } else if counts.len() == 3 {
        for (_, count) in counts.iter() {
            if *count == 3 {
                rank = 4;
                break;
            } else if *count == 2 {
                rank = 5;
                break;
            }
        }
    // 2 of a kind
    } else if counts.len() == 4 {
        rank = 6;
    } else {
        rank = 7;
    }
    rank
}

fn update_cards(cards: &[Card; 5]) -> [Card; 5] {
    let mut contains_joker = false;
    for card in cards.iter() {
        if card.value == 'J' {
            contains_joker = true;
            break;
        }
    }

    if !contains_joker {
        return *cards;
    }

    let counts = get_count_map(cards);

    let card_with_most_count;
    let mut sorted_counts = counts.iter().collect::<Vec<(&char, &u64)>>();
    sorted_counts.sort_by(|a, b| a.1.cmp(b.1));

    if sorted_counts[sorted_counts.len() - 1].0 == &'J' {
        if sorted_counts.len() == 1 {
            card_with_most_count = 'A';
        } else {
            card_with_most_count = *sorted_counts[sorted_counts.len() - 2].0;
        }
    } else {
        card_with_most_count = *sorted_counts[sorted_counts.len() - 1].0;
    }

    let mut new_cards: [Card; 5] = [Card::new('A'); 5];

    for (index, card) in cards.iter().enumerate() {
        if card.value == 'J' {
            new_cards[index] = Card {
                value: card_with_most_count,
                score: SCORES[&'J'],
            };
        } else {
            new_cards[index] = *card;
        }
    }

    new_cards
}

fn parse_map(input_str: &str) -> Vec<Hand> {
    input_str
        .lines()
        .map(|line| {
            let data = line.split_whitespace().collect::<Vec<&str>>();
            let cards = [
                Card::new(data[0].chars().nth(0).unwrap()),
                Card::new(data[0].chars().nth(1).unwrap()),
                Card::new(data[0].chars().nth(2).unwrap()),
                Card::new(data[0].chars().nth(3).unwrap()),
                Card::new(data[0].chars().nth(4).unwrap()),
            ];
            let bid = data[1].parse::<u64>().unwrap();
            let hand = Hand::new(&cards, bid);
            hand
        })
        .collect::<Vec<Hand>>()
}

fn solution(input_str: &str) -> String {
    let hands = parse_map(input_str);

    let mut updated_hands: Vec<Hand> = Vec::new();
    updated_hands.reserve(hands.len());
    for hand in hands.iter() {
        let updated_cards = update_cards(&hand.cards);
        let updated_hand = Hand::new(&updated_cards, hand.bid);
        updated_hands.push(updated_hand);
    }
    updated_hands.sort();
    updated_hands
        .iter()
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
        assert_eq!(result, "5905");
    }
}
