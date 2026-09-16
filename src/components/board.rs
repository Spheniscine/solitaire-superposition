use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, SkinTrait, rem}, game::{AnimationKey, Board, BoardPos, Card, DepotRole, NUM_DEPOTS, Skin, Suit}};

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

    let button_width = 8f32;

    let pos_x = {
        let w = 7.;
        let left = 50. - (w * (card_width + spacer_x) + button_width) / 2.;
        move |i: usize| {
            left + (card_width + spacer_x) * i as f32
        }
    };

    let pos_y = |i: usize| {
        start_y + (card_height + spacer_y) * i as f32
    };

    let superpositor_pos = Vec2::new(pos_x(5).midpoint(pos_x(6)), pos_y(2) + 6.);

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
            DepotRole::Superpositor => Some(rsx!{
                div {
                    font_size: "0.9em",
                    position: "relative",
                    top: "0.07em",
                    left: "0.05em",
                    Ket{}
                }
            }),
        }
    };

    let is_dashed = |depot: usize| {
        DepotRole::role(depot) == Some(DepotRole::Superpositor)
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
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            for depot in 0..NUM_DEPOTS {
                if let Some(hint) = get_hint(depot) {
                    CardFrame { 
                        position: get_pos(depot, 0),
                        width: card_width,
                        hint,
                        dashed: is_dashed(depot),
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, !0))
                        },
                    }
                }

                for i in 0..board.depots[depot].len() {
                    if board.selected == Some(BoardPos::new(depot, i)) {
                        div {
                            position: "absolute",
                            top: rem(get_pos(depot, i).y),
                            left: rem(get_pos(depot, i).x),
                            width: rem(card_width),
                            height: rem(selected_height),
                            background_color: "#ff0",
                            border_radius: rem(card_width * CARD_BORDER_RADIUS_RATIO),
                            class: "selected-halo",
                        }
                    }

                    CardComponent { 
                        position: get_pos(depot, i),
                        width: card_width,
                        card: board.depots[depot][i],
                        // number_hint: if !is_face_up(depot) {i + 1},
                        skin,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, i))
                        },
                        ondoubleclick: move |_| {
                            ondoubleclick.call(BoardPos::new(depot, i))
                        },
                        oncontextmenu: move |ev: Event<MouseData>| {
                            ev.prevent_default();
                            oncontextmenu.call(BoardPos::new(depot, i))
                        },
                    }
                }
            }
        }
    }
}