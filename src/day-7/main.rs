use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::fs;

fn main() {
    println!("Hello Day 7!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let hands: Vec<Hand> = input.lines().map(Hand::parse).collect();

    let total_winnings = winnings(&hands);
    println!("Total winnings of hands: {total_winnings}");
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Card(u32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Rank {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    // Sort by rank, then by card (high value)
    rank: Rank,
    cards: [Card; 5],
    bid: u32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card({})", self.letter())
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Card {
    fn parse(card: char) -> Card {
        let value = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1, // Part 2: Joker is now worth 1 point
            'T' => 10,
            '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' => card.to_digit(10).unwrap(),
            _ => panic!("Could not parse card value '{}'", card),
        };
        Card(value)
    }

    fn value(&self) -> u32 {
        self.0
    }

    fn letter(&self) -> char {
        match self.0 {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            1 => 'J', // Part 2: Joker is now worth 1 point
            10 => 'T',
            9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 => char::from_digit(self.0, 10).unwrap(),
            _ => panic!("Unknown Card Contents"),
        }
    }
}

impl Hand {
    fn parse(line: &str) -> Hand {
        let mut parts = line.split(" ");

        let cards: [Card; 5] = parts
            .next()
            .expect("Could not find Cards")
            .chars()
            .map(Card::parse)
            .collect::<Vec<Card>>()
            .try_into()
            .expect("Could not parse Cards");

        let bid = parts
            .next()
            .expect("Could not find bid")
            .parse()
            .expect("Could not parse bid");

        let rank = Self::rank(&cards);

        Hand { cards, bid, rank }
    }

    fn rank(cards: &[Card; 5]) -> Rank {
        let mut counts: HashMap<Card, u32> = cards.iter().fold(HashMap::new(), |mut map, card| {
            if let Some(count) = map.get_mut(card) {
                *count += 1;
            } else {
                map.insert(card.clone(), 1);
            };
            map
        });

        let jokers: u32 = counts.remove(&Card::parse('J')).unwrap_or(0);

        // sort in decreasing value order
        let mut sorted: Vec<u32> = counts.into_values().collect();
        sorted.sort();
        sorted.reverse();
        let mut sorted_iter = sorted.into_iter();

        match (sorted_iter.next(), sorted_iter.next()) {
            // In value order
            (Some(a), _) if a + jokers == 5 => Rank::Five,
            (Some(a), _) if a + jokers == 4 => Rank::Four,
            (Some(a), Some(b)) if a + jokers == 3 && b == 2 => Rank::FullHouse,
            (Some(a), _) if a + jokers == 3 => Rank::Three,
            (Some(a), Some(b)) if a + jokers == 2 && b == 2 => Rank::TwoPair,
            (Some(a), _) if a + jokers == 2 => Rank::OnePair,
            // If only jokers, count Jokers
            (None, None) => match jokers {
                5 => Rank::Five,
                4 => Rank::Four,
                3 => Rank::Three,
                2 => Rank::OnePair,
                _ => Rank::HighCard,
            },
            _ => Rank::HighCard,
        }
    }
}

fn winnings(hands: &[Hand]) -> u32 {
    let mut sorted_hands: Vec<&Hand> = hands.iter().collect();
    sorted_hands.sort();

    sorted_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + (index as u32 + 1) * hand.bid)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEXT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_one() {
        // let hands: Vec<Hand> = TEXT.lines().map(Hand::parse).collect();

        // assert_eq!(winnings(&hands), 6440);
    }

    #[test]
    fn test_part_two() {
        let hands: Vec<Hand> = TEXT.lines().map(Hand::parse).collect();

        assert_eq!(winnings(&hands), 5905);
    }
}
