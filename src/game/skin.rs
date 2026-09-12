use std::ops::Not;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};

use crate::game::Suit;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum RankSkin {
    #[default]
    Numbers,
    Traditional,
}

impl RankSkin {
    pub fn rank_text(self, rank: u8) -> String {
        match self {
            RankSkin::Numbers => rank.to_string(),
            RankSkin::Traditional => {
                match rank {
                    1 => String::from("A"),
                    11 => String::from("J"),
                    12 => String::from("Q"),
                    13 => String::from("K"),
                    _ => rank.to_string(),
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum SuitSkin {
    #[default]
    Animals,
    Shapes,
    Traditional,
}

pub const SYMBOLS_2_FONT_STR: &str = "'Noto Sans Symbols 2'";
pub const KATEX_SUITS_FONT_STR: &str = "KaTeX_Suits";

impl SuitSkin {
    pub fn suit_symbol(self, suit: Suit) -> &'static str {
        match self {
            SuitSkin::Animals => {
                match suit {
                    Suit::Clubs => "🐰",
                    Suit::Diamonds => "🦁",
                    Suit::Hearts => "🦊",
                    Suit::Spades => "🐧",
                }
            },
            SuitSkin::Shapes => {
                match suit {
                    Suit::Clubs => "▲",
                    Suit::Diamonds => "⬥",
                    Suit::Hearts => "●",
                    Suit::Spades => "★",
                }
            }
            SuitSkin::Traditional => {
                match suit {
                    Suit::Clubs => "♣",
                    Suit::Diamonds => "♦︎",
                    Suit::Hearts => "♥",
                    Suit::Spades => "♠",
                }
            }
        }
    }

    pub fn font(self) -> &'static str {
        match self {
            SuitSkin::Animals => "'Noto Color Emoji'",
            SuitSkin::Shapes => SYMBOLS_2_FONT_STR,
            SuitSkin::Traditional => KATEX_SUITS_FONT_STR, // links to custom version of Katex/MLModern that has filled card suits
        }
    }
}

const COLOR_AMBER: [&str; 2] = ["#b70", "#ffb433"];
const COLOR_GREEN: [&str; 2] = ["#062", "#00ff55"];
const COLOR_RED: [&str; 2] = ["#f00", "#ff8888"];
const COLOR_BLUE: [&str; 2] = ["#00d", "#aaaaff"];
const COLOR_BLACK: [&str; 2] = ["#000", "#fff"];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorMode {
    Dark, #[default] Light
}

impl Not for ColorMode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            ColorMode::Dark => ColorMode::Light,
            ColorMode::Light => ColorMode::Dark,
        }
    }
}

impl ColorMode {
    pub fn choose<T>(self, light: T, dark: T) -> T {
        if self == ColorMode::Light {light} else {dark}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorSkin {
    #[default]
    #[strum(to_string = "Four colors")]
    FourColor,
    #[strum(to_string = "Two colors")]
    TwoColor,
}

impl ColorSkin {
    pub fn color(self, suit: Suit, mode: ColorMode) -> &'static str {
        let res = match self {
            ColorSkin::FourColor => {
                // Use Spectrum Bridge colors - better distinction between reddish/warm and blackish/cool colors for
                // solitaires that care about that
                match suit {
                    Suit::Clubs => COLOR_GREEN,
                    Suit::Diamonds => COLOR_AMBER,
                    Suit::Hearts => COLOR_RED,
                    Suit::Spades => COLOR_BLUE,
                }
            },
            ColorSkin::TwoColor => {
                match suit {
                    Suit::Clubs | Suit::Spades => COLOR_BLACK,
                    Suit::Diamonds | Suit::Hearts => COLOR_RED,
                }
            },
        };
        res[mode as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct Skin {
    pub ranks: RankSkin,
    pub suits: SuitSkin,
    pub colors: ColorSkin,
}