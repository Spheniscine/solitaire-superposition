use std::ops::Range;

use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use strum::{IntoEnumIterator, VariantArray};
use strum_macros::{EnumIter, VariantArray};

use crate::game::NUM_SUITS;

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