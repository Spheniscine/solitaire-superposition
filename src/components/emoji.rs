use dioxus::{logger::tracing, prelude::*};
use phf::phf_map;

pub static EMOJI_MAP: phf::Map<&'static str, Asset> = phf_map! {
    "🐰" => asset!("/assets/emoji/emoji_u1f430.svg"),
    "🦁" => asset!("/assets/emoji/emoji_u1f981.svg"),
    "🦊" => asset!("/assets/emoji/emoji_u1f98a.svg"),
    "🐧" => asset!("/assets/emoji/emoji_u1f427.svg"),
};

#[component]
pub fn Emoji(text: String) -> Element {
    if let Some(asset) = EMOJI_MAP.get(&text) {
        rsx! {
            img {
                style: "height: 1.175em; vertical-align: middle;",
                src: *asset,
                draggable: false,
                alt: text,
            }
        }
    } else {
        tracing::error!("No emoji asset loaded for string '{text}'");
        rsx! {
            "ERROR"
        }
    }
    
}