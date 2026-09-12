use std::ops::Range;

use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use strum::{IntoEnumIterator, VariantArray};
use strum_macros::{EnumIter, VariantArray};

use crate::game::{Card, DECK_SIZE, NUM_SUITS};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, EnumIter, VariantArray)]
#[repr(u8)]
pub enum DepotRole {
    Tableau,
    Foundation,
    Superpositor,
}

pub const NUM_DEPOTS: usize = {
    let mut sum = 0;
    let mut index = 0;
    while index < DepotRole::VARIANTS.len() {
        sum += DepotRole::VARIANTS[index].number_of();
        index += 1;
    }
    sum
};

impl DepotRole {
    pub const fn number_of(&self) -> usize {
        match self {
            DepotRole::Tableau => 4,
            DepotRole::Foundation => NUM_SUITS,
            DepotRole::Superpositor => 1,
        }
    }

    pub const fn offset(self) -> usize {
        let mut sum = 0;
        let mut index = 0;
        loop {
            if index == self as usize { return sum; }
            sum += DepotRole::VARIANTS[index].number_of();
            index += 1;
        }
    }

    pub const fn range(self) -> Range<usize> {
        self.offset() .. self.offset() + self.number_of()
    }

    pub fn role_and_subindex(i: usize) -> Option<(DepotRole, usize)> {
        for role in Self::iter() {
            if role.range().contains(&i) {
                return Some((role, i - role.offset()))
            }
        }
        None
    }

    pub fn role(i: usize) -> Option<DepotRole> {
        Self::role_and_subindex(i).map(|x| x.0)
    }

    pub fn id(self, i: usize) -> usize {
        self.offset() + i
    }
}

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple, Debug, PartialEq, Eq)]
pub struct BoardPos {
    pub depot_index: usize,
    pub card_index: usize,
}

impl BoardPos {
    pub fn new(depot_index: usize, card_index: usize) -> Self {
        Self { depot_index, card_index }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AnimationAct {
    Move { cards: Vec<Card>, pos1: BoardPos, pos2: BoardPos },
    Split { card: Card, pos: BoardPos, undo: bool },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Board {
    pub depots: Vec<Vec<Card>>,
    pub selected: Option<BoardPos>,
    pub num_splits_remaining: i8,
    pub animation_acts: Vec<AnimationAct>,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            depots: vec![vec![]; NUM_DEPOTS],
            selected: None,
            num_splits_remaining: 4,
            animation_acts: vec![],
        }
    }

    pub fn from_deal(deal: &[Card]) -> Self {
        use DepotRole::*;
        assert_eq!(deal.len(), DECK_SIZE);

        let mut res = Self::empty();
        for (&card, depot) in deal.iter().zip(std::iter::repeat(Tableau.range()).flatten()) {
            res.depots[depot].push(card);
        }

        res
    }

    pub fn do_move(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.selected = None;
        let cards = self.depots[pos1.depot_index].drain(pos1.card_index ..).collect::<Vec<_>>();
        self.animation_acts.push(
            AnimationAct::Move { 
                cards, pos1, pos2
            }
        );
    }

    pub fn reset_selection(&mut self) {
        self.selected = None;
        if !self.depots[DepotRole::Superpositor.id(0)].is_empty() {
            self.selected = Some(BoardPos::new(DepotRole::Superpositor.id(0), 0));
        }
    }

    pub fn do_split(&mut self, pos: BoardPos, undo: bool) {
        self.selected = None;
        if !undo {
            self.depots[pos.depot_index][pos.card_index].tapped = true;
            let card = self.depots[pos.depot_index][pos.card_index];
            
            self.animation_acts.push(AnimationAct::Split { card, pos, undo });
            self.num_splits_remaining -= 1;
        } else {
            let card = self.depots[DepotRole::Superpositor.id(0)].pop().unwrap();

            self.animation_acts.push(AnimationAct::Split { card, pos, undo });
        }
    }

    pub fn advance_actions(&mut self) {
        for act in self.animation_acts.drain(..) {
            match act {
                AnimationAct::Move { cards, pos2, .. } => {
                    self.depots[pos2.depot_index].extend(cards);
                },
                AnimationAct::Split { card, pos, undo } => {
                    if !undo {
                        self.depots[DepotRole::Superpositor.id(0)].push(card);
                    } else {
                        self.depots[pos.depot_index][pos.card_index].tapped = false;
                        self.num_splits_remaining += 1;
                    }
                },
            }
        }
        self.reset_selection();
    }

    pub fn top_pos(&self, depot: usize) -> BoardPos {
        BoardPos::new(depot, self.depots[depot].len())
    }

    pub fn last_pos(&self, depot: usize) -> BoardPos {
        BoardPos::new(depot, self.depots[depot].len().wrapping_sub(1))
    }
}