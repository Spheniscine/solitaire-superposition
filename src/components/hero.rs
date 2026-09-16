use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::BoardComponent, game::{GameState, ScreenState}};

#[component]
pub fn Hero() -> Element {
    let mut state = use_signal(|| {
        // if let Some(mut state) = LocalStorage.load_game_state() {
        //     state.reset_selection();
        //     state.screen_state = ScreenState::Game;
        //     return state;
        // }
        GameState::init()
    });
    let confetti_counter = use_memo(move || {
        state.read().num_wins
    });
    use_effect(move || {
        let _ = confetti_counter.read();
        document::eval("confetti();");
    });

    let st = state.read();
    // let clean = !st.is_busy(); // interactions should test this before write()-ing to state, to prevent slowdowns

    // let animate_timer = use_coroutine(move |mut rx: UnboundedReceiver<AnimationKey>| async move {
    //     while let Some(key) = rx.next().await {
    //         async_std::task::sleep(ANIMATION_DURATION).await;
    //         state.write().advance_animations(key);
    //     }
    // });

    // if st.is_acting() {
    //     animate_timer.send(st.animation_key);
    // }

    rsx! {
        div {
            id: "hero",
            class: "select-none",
            if st.screen_state == ScreenState::Game {

                BoardComponent { 
                    position: Vec2 { x: 0., y: 20. },
                    board: st.board.clone(),
                    skin: st.skin,
                    // onclick: move |pos| if clean {state.write().onclick(pos);},
                    // ondoubleclick: move |pos| if clean {state.write().ondoubleclick(pos);},
                    // oncontextmenu: move |pos| if clean {state.write().oncontextmenu(pos);},
                    // animation_key: st.animation_key,
                    // is_won: st.is_won(),
                }
            }
        }
    }
}