use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::game::{ColorSkin, GameState, RankSkin, ScreenState, SuitSkin};

#[component]
pub fn Settings(game_state: Signal<GameState>) -> Element {
    let mut state = use_signal(|| {
        game_state.read().new_settings_state()
    });
    let mut ok = move || {
        game_state.write().apply_settings(&state.read());
        game_state.write().screen_state = ScreenState::Game;
    };
    let mut cancel = move || {
        game_state.write().screen_state = ScreenState::Game;
    };

    let onmounted = async move |e: Event<MountedData>| {
        let _ = e.set_focus(true).await;
    };
    let onkeydown = move |e: Event<KeyboardData>| {
        let key = e.key();
        match key {
            Key::Enter => {
                ok();
            }
            Key::Escape => {
                cancel();
            }
            _ => {}
        }
    };

    let allow_undo_changed = move |evt: Event<FormData>| {
        state.write().allow_undo = evt.checked();
    };

    let rank_skin_changed = move |evt: Event<FormData>| {
        let v = evt.value().parse().ok().and_then(|v| { RankSkin::from_repr(v) });
        state.write().skin.ranks = v.unwrap_or_default();
    };
    let suit_skin_changed = move |evt: Event<FormData>| {
        let v = evt.value().parse().ok().and_then(|v| { SuitSkin::from_repr(v) });
        state.write().skin.suits = v.unwrap_or_default();
    };
    let color_skin_changed = move |evt: Event<FormData>| {
        let v = evt.value().parse().ok().and_then(|v| { ColorSkin::from_repr(v) });
        state.write().skin.colors = v.unwrap_or_default();
    };

    rsx! {
        div {
            id: "settingsDialog",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,

            p {
                "Allow undo/reset: "
                input {
                    r#type: "checkbox",
                    checked: state.read().allow_undo,
                    onchange: allow_undo_changed,
                }
            }

            p {
                "Card style: "
            }

            div {
                margin_left: "5rem",
                p {
                    "Ranks: "
                    select {
                        onchange: rank_skin_changed,
                        for x in RankSkin::iter() {
                            option {
                                value: x as usize,
                                selected: state.read().skin.ranks == x,
                                "{x}"
                            }
                        }
                    }
                }
                p {
                    "Suits: "
                    select {
                        onchange: suit_skin_changed,
                        for x in SuitSkin::iter() {
                            option {
                                value: x as usize,
                                selected: state.read().skin.suits == x,
                                "{x}"
                            }
                        }
                    }
                }
                p {
                    "Color scheme: "
                    select {
                        onchange: color_skin_changed,
                        for x in ColorSkin::iter() {
                            option {
                                value: x as usize,
                                selected: state.read().skin.colors == x,
                                "{x}"
                            }
                        }
                    }
                }
            }

            p {
                button {
                    r#type: "button",
                    onclick: move |_| ok(),
                    "OK"
                }
                " ",
                button {
                    r#type: "button",
                    onclick: move |_| cancel(),
                    "Cancel"
                }
            }

            p {
                class: "copyright",
                "Game rules: Tear Solitaire” by Hempuli", br{},
                "Webapp © OnlineMathLearning.com"
            }
        }
    }
}