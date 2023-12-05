use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!("Hello Day 4!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let cards = input
        .lines()
        .map(SctratchCard::parse)
        .collect::<Vec<SctratchCard>>();

    let sum_scores = sum_scratchcards_points(&cards);
    println!("Scratch Cards score sum is: {sum_scores}");

    let total_scratchcards = sum_won_scratchcards(&cards);
    println!("Winning Scratch Cards total points is: {total_scratchcards}");
}

#[derive(Debug)]
struct SctratchCard {
    id: u32,
    // Array sizes derived from input.txt structure
    winning: HashSet<u32>,
    provided: HashSet<u32>,
}

impl SctratchCard {
    fn parse(line: &str) -> SctratchCard {
        // Expect line to be in the following format
        // Card <num>: <num> ... | <num> ...
        let mut parts = line.split(":");
        let id = if let Some(card_id) = parts.next() {
            card_id
                .split(" ")
                .find(|id| id.parse::<u32>().is_ok())
                .expect("Card id not found")
                .parse::<u32>()
                .expect("Card id not a number")
        } else {
            // We could return Result<>, but this is easy
            dbg!(line);
            panic!("Could not parse line");
        };

        let mut numbers = parts.next().expect("Numbers not found").split(" | ");

        let winning = numbers
            .next()
            .expect("Winning numbers not found")
            .split(" ")
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<HashSet<u32>>();

        let provided = numbers
            .next()
            .expect("Provided numbers not found")
            .split(" ")
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<HashSet<u32>>();

        SctratchCard {
            id,
            winning,
            provided,
        }
    }

    fn winning_numbers(&self) -> HashSet<u32> {
        self.winning
            .intersection(&self.provided)
            .cloned()
            .collect::<HashSet<u32>>()
    }

    fn score(&self) -> u32 {
        // a score is every provided number found in winning numbers
        // the points scored is 1 for the first number, then doubled for every sucessive number
        let wins = self.winning_numbers().len();
        if wins == 0 {
            0
        } else {
            2_u32.pow(wins as u32 - 1)
        }
    }
}

fn sum_scratchcards_points(cards: &[SctratchCard]) -> u32 {
    cards.iter().map(|card| card.score()).sum()
}

fn sum_won_scratchcards(cards: &[SctratchCard]) -> u32 {
    // For each winning card, for the count of winning numbers, gain a copy of id + winning_num_index + 1 card
    // Sum is score of all winning, and copied, cards

    cards
        .iter()
        .fold(HashMap::<u32, u32>::new(), |mut acc, card| {
            let self_copies = acc.get(&card.id).unwrap_or(&0) + 1;
            acc.insert(card.id, self_copies);

            // Add coppies from winning numbers
            let card_ids = card
                .winning_numbers()
                .iter()
                .enumerate()
                .map(|(i, _)| card.id + i as u32 + 1)
                .collect::<HashSet<u32>>();
            for target_id in card_ids {
                let target_copies = acc.get(&target_id).unwrap_or(&0);
                acc.insert(target_id, target_copies + 1 * self_copies);
            }

            acc
        })
        .values()
        .sum()
}

#[test]
fn test_sum_scratchcards_points() {
    let cards = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .lines()
        .map(SctratchCard::parse)
        .collect::<Vec<SctratchCard>>();

    assert_eq!(sum_scratchcards_points(&cards), 13)
}

#[test]
fn test_test_sum_scratchcards_points() {
    let cards = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .lines()
        .map(SctratchCard::parse)
        .collect::<Vec<SctratchCard>>();

    assert_eq!(sum_won_scratchcards(&cards), 30)
}
