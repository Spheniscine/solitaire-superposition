use std::ops::RangeInclusive;

use enum_map::Enum;
use serde::{Deserialize, Serialize, de::Visitor};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumCount, EnumIter, Enum)]
pub enum Suit {
    Clubs, Diamonds, Hearts, Spades
}

impl Suit {
    pub fn code(self) -> char {
        match self {
            Suit::Clubs => 'C',
            Suit::Diamonds => 'D',
            Suit::Hearts => 'H',
            Suit::Spades => 'S',
        }
    }
    pub fn from_code(c: char) -> Option<Self> {
        match c {
            'C' => Some(Suit::Clubs),
            'D' => Some(Suit::Diamonds),
            'H' => Some(Suit::Hearts),
            'S' => Some(Suit::Spades),
            _ => None,
        }
    }
}

impl Serialize for Suit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_char(self.code())
    }
}

impl<'de> Deserialize<'de> for Suit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MyVisitor;
        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Suit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "suit code, one of characters CHDS")
            }
            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
                where E: serde::de::Error, {
                Suit::from_code(v).ok_or_else(|| E::custom(format!("invalid suit code: {}", v)))
            }
        }
        deserializer.deserialize_char(MyVisitor)
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit, 
    pub tapped: bool,
}

pub const RANK_MIN: u8 = 1;
pub const RANK_MAX: u8 = 13;
pub const RANKS: RangeInclusive<u8> = RANK_MIN ..= RANK_MAX;
pub const NUM_SUITS: usize = Suit::COUNT;
pub const NUM_RANKS: usize = (RANK_MAX - RANK_MIN) as usize + 1;
pub const DECK_SIZE: usize = NUM_RANKS * NUM_SUITS;

impl Card {
    pub fn code(self) -> String {
        format!("{}{}{}", self.rank, self.suit.code(), if self.tapped {"X"} else {""})
    }
    pub fn from_code(mut code: &str) -> Option<Self> {
        let tapped = code.strip_suffix("X").is_some_and(|x| {
            code = x;
            true
        });
        let mut it = code.chars();
        let suit = Suit::from_code(it.next_back()?)?;
        let rank: u8 = it.as_str().parse().ok()?;
        if !RANKS.contains(&rank) { return None; }
        Some(Card { rank, suit, tapped })
    }
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.code())
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MyVisitor;
        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Card;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "suit code, one of characters CHDS")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Card::from_code(v).ok_or_else(|| E::custom(format!("invalid card code: {}", v)))
            }
        }
        deserializer.deserialize_str(MyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Card, Suit};

    #[test]
    fn card_to_code_test() {
        let card = Card {
            rank: 13, suit: Suit::Spades, tapped: false
        };
        assert_eq!("13S", card.code())
    }

    #[test]
    fn card_from_code_test() {
        let card = Card {
            rank: 13, suit: Suit::Spades, tapped: false
        };
        assert_eq!(Some(card), Card::from_code("13S"))
    }

    #[test]
    fn card_to_code_tapped_test() {
        let card = Card {
            rank: 13, suit: Suit::Spades, tapped: true
        };
        assert_eq!("13SX", card.code())
    }

    #[test]
    fn card_from_code_tapped_test() {
        let card = Card {
            rank: 13, suit: Suit::Spades, tapped: true
        };
        assert_eq!(Some(card), Card::from_code("13SX"))
    }
}