use dioxus::prelude::*;
use glam::Vec2;
use math_macro::math;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, EMOJI_MAP, Movement, SkinTrait, rem}, game::{AnimationAct, AnimationKey, Board, BoardPos, Card, DepotRole, NUM_DEPOTS, Skin, Suit}};

// symbol used for superpositor
#[component]
pub fn Ket() -> Element {
    rsx! {
        span {
            dangerous_inner_html: math!(r"|\phi\rangle"),
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
    onclick_auto_play: EventHandler<()>,
    #[props(default)]
    auto_play: bool,

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
                    left: "0.04em",
                    Ket{}
                }
            }),
        }
    };

    let is_dashed = |depot: usize| {
        DepotRole::role(depot) == Some(DepotRole::Superpositor)
    };

    let number_hint = |depot: usize| {
        if DepotRole::role(depot) == Some(DepotRole::Superpositor) {
            Some(board.num_splits_remaining as i32)
        } else {None}
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = if DepotRole::role(depot_index).unwrap() == DepotRole::Tableau {
            board.depots[depot_index].len() - card_index - 1
        } else {
            0
        };

        card_height + column_card_offset.y * d as f32
    } else {0.};

    let auto_play_button = {
        let pos = Vec2::new(
            pos_x(7),
            pos_y(0) + card_height + (spacer_y - button_width) / 2.
        );

        rsx! {
            img { 
                style: "width: {rem(button_width)}; position: absolute; left: {rem(pos.x)}; top: {rem(pos.y)}",
                class: if auto_play {"selected-halo"},
                onclick: move |_| {onclick_auto_play.call(())},
                src: EMOJI_MAP["⏫"]
            }
        }
    };

    let moving_card = |p1: Vec2, p2: Vec2, card: Card| rsx! {
        Movement {
            src_translate_vec: p1 - p2,
            CardComponent {
                position: p2,
                width: card_width,
                card: card,
                skin,
            }
        }
    };

    let anims = board.animation_acts.iter().enumerate().map(|(i, act)| {
        match act {
            AnimationAct::Move { cards, pos1, pos2 } => {
                let mut pos1 = *pos1;
                let mut pos2 = *pos2;

                let nodes = cards.iter().map(move |card| {
                    let p1 = get_pos(pos1.depot_index, pos1.card_index);
                    let p2 = get_pos(pos2.depot_index, pos2.card_index);
                    let res = moving_card(p1, p2, *card);
                    pos1.card_index += 1;
                    pos2.card_index += 1;
                    res
                });

                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        {nodes}
                    }
                }
            },
            AnimationAct::Split { card, pos, undo } => {
                let p1 = get_pos(pos.depot_index, pos.card_index);
                let p2 = get_pos(DepotRole::Superpositor.id(0), 0);
                let node = if !undo {
                    moving_card(p1, p2, *card)
                } else {
                    moving_card(p2, p1, *card)
                };

                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        {node}
                    }
                }
            },
        }
    });

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            {auto_play_button}

            for depot in 0..NUM_DEPOTS {
                if let Some(hint) = get_hint(depot) {
                    CardFrame { 
                        position: get_pos(depot, 0),
                        width: card_width,
                        hint,
                        dashed: is_dashed(depot),
                        number_hint: number_hint(depot),
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

            {anims}

            if is_won {
                div {
                    position: "absolute",
                    top: rem(25.),
                    left: rem(17.5),
                    width: rem(59.),
                    background_color: "#505",
                    padding: rem(3.),
                    color: "#fff",
                    font_size: rem(7.),
                    border_radius: rem(2.),
                    text_align: "center",
                    "YOU WIN!",
                }
            }
        }
    }
}