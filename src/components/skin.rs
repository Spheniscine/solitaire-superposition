use dioxus::prelude::*;

use crate::{components::{Emoji, SkinTrait}, game::{Card, ColorMode, KATEX_SUITS_FONT_STR, SYMBOLS_2_FONT_STR, Skin, SuitSkin}};

impl Skin {
    fn render_suit_internal(&self, card: &Card, text_mode: bool) -> Element {
        if self.suits == SuitSkin::Animals {
            rsx! {
                Emoji { 
                    text: self.suits.suit_symbol(card.suit)
                }
            }
        } else {
            let font = self.suits.font();
            rsx! {
                span {
                    font_family: font,
                    position: if !text_mode && font == SYMBOLS_2_FONT_STR {"relative"},
                    top: if !text_mode && font == SYMBOLS_2_FONT_STR {"0.13em"},
                    {self.suits.suit_symbol(card.suit)}
                }
            }
        }
    }
}

impl SkinTrait<Card> for Skin {
    fn get_color(&self, card: &Card, mode: ColorMode) -> String {
        self.colors.color(card.suit, mode).to_string()
    }

    fn render_rank(&self, card: &Card) -> Element {
        rsx! {
            span {
                font_family: KATEX_SUITS_FONT_STR,

                "{card.rank}"
            }
        }
    }

    fn render_suit(&self, card: &Card) -> Element {
        self.render_suit_internal(card, false)
    }

    fn render_suit_text(&self, card: &Card) -> Element {
        self.render_suit_internal(card, true)
    }
}