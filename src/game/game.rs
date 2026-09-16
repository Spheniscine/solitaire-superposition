use std::time::Duration;

use enum_map::EnumMap;
use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{components::LocalStorage, game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, NUM_RANKS, RANKS, Skin, Suit}};

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ActionRecord {
    Move { pos1: BoardPos, pos2: BoardPos },
    Split { pos: BoardPos },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub auto_play: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit, tapped: false });
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            auto_play: false,
            skin: Skin::default(),
        };

        res.new_game();
        res
    }

    pub fn new_game(&mut self) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;
        LocalStorage.save_game_state(&self);
    }

    pub fn reset_selection(&mut self) {
        self.board.reset_selection();
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.board.do_move(pos1, pos2);
        self.history.push(ActionRecord::Move { pos1, pos2 })
    }

    fn do_split_raw(&mut self, pos: BoardPos) {
        self.board.do_split(pos, false);
        self.history.push(ActionRecord::Split { pos })
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        !(back.tapped && front.tapped) &&
        back.suit != front.suit &&
        (back.tapped || front.tapped || back.rank == front.rank + 1)
    }

    pub fn can_sort(&self, back: Card, front: Card) -> bool {
        back.suit == front.suit && back.rank + 1 == front.rank
    }

    pub fn can_select(&self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        let Some(role) = DepotRole::role(depot) else { return false };
        match role {
            DepotRole::Tableau => {
                self.board.depots[DepotRole::Superpositor.id(0)].is_empty() &&
                slice.windows(2).all(|w| self.can_stack(w[0], w[1]))
            },
            DepotRole::Foundation => false,
            DepotRole::Superpositor => true,
        }
    }

    pub fn move_intent(&mut self, pos1: BoardPos, pos2: BoardPos) -> bool {
        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        let num_moved = depot1.len() - pos1.card_index;
        if pos2.card_index != depot2.len() { return false; }

        let Some(role) = DepotRole::role(pos2.depot_index) else { return false };
        let history_len = self.history.len();

        let card = depot1[pos1.card_index];

        match role {
            DepotRole::Tableau => {
                let ok = depot2.last().is_none_or(|&c| self.can_stack(c, card));
                if !ok { return false; }
                self.do_move_raw(pos1, pos2);
            },
            DepotRole::Foundation => {
                if num_moved != 1 { return false; }
                let ok = depot2.last().is_none_or(|&c| self.can_sort(c, card));
                if !ok { return false; }
                self.do_move_raw(pos1, pos2);
            },
            DepotRole::Superpositor => {
                if num_moved != 1 || !depot2.is_empty() || 
                    self.board.num_splits_remaining <= 0 || card.tapped { return false; }
                self.do_split_raw(pos1);
            },
        }

        self.undo_stack.push(history_len);
        true
    }

    pub fn is_won(&self) -> bool {
        use DepotRole::*;
        Foundation.range().all(|d| self.board.depots[d].len() == NUM_RANKS)
    }

    pub fn is_over(&self) -> bool {
        self.is_won()
    }

    fn undo_with_override(&mut self, overr: bool) {
        if self.is_busy() || !(overr || self.undo_possible()) { return; }
        let Some(target_len) = self.undo_stack.pop() else {return};
        while self.history.len() > target_len {
            let rec = self.history.pop().unwrap();
            match rec {
                ActionRecord::Move { pos1, pos2 } => {
                    self.board.do_move(pos2, pos1)
                },
                ActionRecord::Split { pos } => {
                    self.board.do_split(pos, true);
                },
            }
            if !overr { self.board.advance_actions(); } // no animation, as repeated card moves on same card causes problems
        }
    }

    pub fn undo(&mut self) {
        self.undo_with_override(false);
        LocalStorage.save_game_state(&self);
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_over() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                if DepotRole::role(pos.depot_index) == Some(DepotRole::Superpositor) {
                    self.undo_with_override(true);
                    return;
                }
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            let dest = BoardPos { depot_index: pos.depot_index, card_index: pos.card_index.wrapping_add(1) };
            self.move_intent(src, dest);
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    fn get_next_sort(&self) -> Option<BoardPos> {
        if !self.board.depots[DepotRole::Superpositor.id(0)].is_empty() { return None; }
        let mut foundation_ranks = EnumMap::<Suit, u8>::default();
        for i in DepotRole::Foundation.range() {
            if let Some(card) = self.board.depots[i].last() {
                foundation_ranks[card.suit] = card.rank;
            }
        }

        DepotRole::Tableau.range().filter_map(|depot| {
            if let Some(&card) = self.board.depots[depot].last() {
                if foundation_ranks[card.suit] + 1 == card.rank {
                    return Some((card.rank, self.board.last_pos(depot)))
                }
            }
            None
        }).min_by_key(|x| x.0).map(|x| x.1)
    }

    fn try_sort(&mut self, pos: BoardPos) {
        for dest in DepotRole::Foundation.range() {
            let dest = self.board.top_pos(dest);
            if self.move_intent(pos, dest) {
                return;
            }
        }
    }

    pub fn check_auto_moves(&mut self) {
        if self.is_busy() { return; }
        if self.is_over() { return; }
        if !self.auto_play { return; }

        if let Some(pos) = self.get_next_sort() {
            self.try_sort(pos);
        } else {
            self.auto_play = false;
        }
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            self.check_auto_moves();
        }

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn restart(&mut self) {
        if self.history.is_empty() || !self.undo_possible() { return; }
        self.board = Board::from_deal(&self.deal);
        self.history.clear();
        self.undo_stack.clear();

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }
}