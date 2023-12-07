use std::{cmp::Ordering, path::Path, str::FromStr};

use aoc_utils::get_entire_puzzle;
use counter::Counter;

fn main() {
    let result = total_winnings("input");
    println!("Total winnings are: {result}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(input: &str) -> Result<Card, Self::Err> {
        match input {
            "J" => Ok(Self::Joker),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cards(Vec<Card>);

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        for (card, other_card) in self.0.iter().zip(&other.0) {
            if card == other_card {
                continue;
            } else {
                match card < other_card {
                    true => return Ordering::Less,
                    false => return Ordering::Greater,
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn from_cards(cards: &Cards) -> Self {
        let mut counter = cards.0.iter().collect::<Counter<_>>();

        let jokers = counter.remove(&Card::Joker).unwrap_or(0);

        let mut counts = counter.most_common();

        if let Some((_, n)) = counts.get_mut(0) {
            *n += jokers;
        }

        let unique_cards = counts.len();

        if unique_cards == 1 || unique_cards == 0 {
            Self::FiveKind
        } else if unique_cards == 5 {
            return Self::High;
        } else if unique_cards == 4 {
            return Self::OnePair;
        } else if unique_cards == 2 {
            if counts.first().unwrap().1 == 4 {
                return Self::FourKind;
            } else {
                return Self::FullHouse;
            }
        } else if counts.first().unwrap().1 == 3 {
            return Self::ThreeKind;
        } else {
            return Self::TwoPair;
        }
    }
}

#[derive(Clone)]
struct Hand {
    cards: Cards,
    type_: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ordering = self.type_.cmp(&other.type_);

        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (&self.cards, &self.type_) == (&other.cards, &other.type_)
    }
}

impl Eq for Hand {}

impl FromStr for Hand {
    type Err = ();

    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        let split_input: Vec<&str> = input.split_ascii_whitespace().collect();

        let cards = split_input.first().unwrap();
        let bid = split_input.last().unwrap();

        let cards: Vec<Card> = cards
            .chars()
            .map(|c| Card::from_str(&c.to_string()).unwrap())
            .collect();
        let bid: usize = bid.parse().unwrap();

        let cards = Cards(cards);
        let type_ = HandType::from_cards(&cards);

        Ok(Self { cards, type_, bid })
    }
}
type Hands = Vec<Hand>;

fn parse_hands(input: impl AsRef<Path>) -> Hands {
    let lines = get_entire_puzzle(input);

    lines
        .into_iter()
        .map(|l| Hand::from_str(&l).unwrap())
        .collect()
}

fn total_winnings(input: impl AsRef<Path>) -> usize {
    let mut hands = parse_hands(input);
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two() {
        let result = total_winnings("test_part1");
        assert_eq!(result, 5905);
    }
}
