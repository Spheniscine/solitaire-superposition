use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_HEIGHT_RATIO, SkinTrait}, game::{AnimationKey, Board, BoardPos, Card, DepotRole, Skin, Suit}};

// symbol used for superpositor
#[component]
pub fn Ket() -> Element {
    rsx! {
        span {
            // |\phi\rangle
            dangerous_inner_html: include_str!("../includes/ket.html")
        }
    }
}

#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
    #[props(default)]
    onclick: EventHandler<BoardPos>,
    #[props(default)]
    ondoubleclick: EventHandler<BoardPos>,
    #[props(default)]
    oncontextmenu: EventHandler<BoardPos>,
    #[props(default)]
    animation_key: AnimationKey,
    #[props(default)]
    is_won: bool,
) -> Element {
    let card_width = 11f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer_x = 1f32;
    let spacer_y = 1f32;
    let start_y = 2f32;

    let pos_x = {
        let w = 7.;
        let left = 50. - (w * card_width + (w-1.) * spacer_x) / 2.;
        move |i: usize| {
            left + (card_width + spacer_x) * i as f32
        }
    };

    let pos_y = |i: usize| {
        start_y + (card_height + spacer_y) * i as f32
    };

    let superpositor_pos = Vec2::new(pos_x(5).midpoint(pos_x(6)), pos_y(4));

    let column_card_offset = Vec2::new(0., 6.);

    let get_pos = |depot: usize, ord: usize| {
        let (role, index) = DepotRole::role_and_subindex(depot).unwrap();
        match role {
            DepotRole::Tableau => 
                Vec2::new(pos_x(index), pos_y(0)) + column_card_offset * ord as f32,
            DepotRole::Foundation => {
                let r = index / 2;
                let c = index % 2;
                Vec2::new(pos_x(5 + c), pos_y(r))
            },
            DepotRole::Superpositor => superpositor_pos,
        }
    };

    let get_hint = |depot: usize| {
        let role = DepotRole::role(depot).unwrap();
        match role {
            DepotRole::Tableau => Some(rsx!{}),
            DepotRole::Foundation => Some(skin.render_rank(&Card { rank: 1, suit: Suit::Spades, tapped: false })),
            DepotRole::Superpositor => Some(rsx!{Ket{}}),
        }
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = if DepotRole::role(depot_index).unwrap() == DepotRole::Tableau {
            board.depots[depot_index].len() - card_index - 1
        } else {
            0
        };

        card_height + column_card_offset.y * d as f32
    } else {0.};

    rsx! {

    }
}