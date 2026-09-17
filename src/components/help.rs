use dioxus::prelude::*;

use crate::{components::{CardText, Emoji, Ket, SkinTrait, VIDEO_GAMEPLAY, rem}, game::{Card, ColorMode, GameState, RankSkin, ScreenState, Suit}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let stack_example = || {
        let mut ite = [
            Card { rank: 5, suit: Suit::Spades, tapped: false },
            Card { rank: 4, suit: Suit::Hearts, tapped: false },
            Card { rank: 3, suit: Suit::Diamonds, tapped: false },
            Card { rank: 2, suit: Suit::Spades, tapped: false },
        ].into_iter().map(|card| {
            rsx! {
                CardText { 
                    card, skin, color_mode: ColorMode::Light,
                }
            }
        });


        let last = ite.next().unwrap();
        rsx! {
            {ite.next().unwrap()},
            for x in ite { "–", {x} },
            " can be placed on ", {last}
        }
    };

    let rank_text = |rank: u8| {
        rsx! {
            span {
                font_size: "1.2em",
                {skin.render_rank(&Card { rank, suit: Suit::Spades, tapped: false })}
            }
        }
    };

    let foundation_req = match skin.ranks {
        RankSkin::Numbers => rsx! {
            {rank_text(1)} " to " {rank_text(13)}
        },
        RankSkin::Traditional => rsx! {
            "Ace to King"
        },
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 3.5rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The ",Emph {"tableau"}," consists of 4 columns. Cards in the tableau are stacked by "
                    Emph{"decrementing ranks"} " and " Emph{"unlike suit"} 
                    ". Such stacks of any size can be moved as a unit. (e.g. ",{stack_example()},")."
                }

                p {
                    "The " Emph {"superpositor"} " (" Ket{} ") can be used to split a card into two " Emph {"eigencards"} "."
                    ul {
                        li {
                            "One of the eigencards will stay in the card’s original location, while the other must be immediately 
                            placed in a valid position."
                        }
                        li {
                            "An eigencard's rank is " Emph {"uncertain"} ", which means its rank is " Emph{"ignored"} " by the 
                            stacking rule in the tableau."
                        }
                        li {
                            "Two eigencards cannot be stacked, and eigencards may not be split again."
                        }
                        li {
                            "Eigencards retain their rank for the purpose of the foundations. One of each pair will go to the 
                            foundations, while the other will remain in the tableau."
                        }
                        li {
                            "The superpositor can only split up to 4 cards; otherwise the game would become unwinnable."
                        }
                    }
                }

                p {
                    "To " Emph{"win the game"} ", stack cards to the " Emph{"foundations"}
                    " in incrementing order from " {foundation_req} " for each suit."
                }

                p {
                    Emph{"Shortcut notes:"},

                    ul {
                        li {
                            Emph {"Right-clicking / long-pressing"} " a card will split it using the superpositor."
                        }
                        li {
                            Emph {"Double-clicking"} " a card will send it to the foundations if possible."
                        }
                        li {
                            "Press the " Emoji { text: "⏫" } " button to " Emph {"auto-play"} " cards to the foundations.
                            Moves made may not necessarily be safe, so you may stop the auto-play at any time, 
                            and undo unwanted moves individually."
                        }
                    }
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",

                    a {
                        href: VIDEO_GAMEPLAY,
                        target: "_blank",
                        text_decoration: "none",
                        margin_right: rem(4.),
                        div {
                            width: rem(30.),
                            position: "relative",
                            class: "game-button",
                            "Example video"
                        }
                    }

                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                        "Back to game"
                    }
                }
            }
        }
    }
}