use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::rem, game::{Card, ColorMode, KATEX_SUITS_FONT_STR, Skin}};

pub trait SkinTrait<C>: PartialEq + Clone {
    fn get_color(&self, card: &C, mode: ColorMode) -> String;
    fn render_rank(&self, card: &C) -> Element;
    fn render_suit(&self, card: &C) -> Element;
    fn render_suit_text(&self, card: &C) -> Element;
}

pub const CARD_HEIGHT_RATIO: f32 = 13. / 12.;
pub const CARD_BORDER_RADIUS_RATIO: f32 = 1.5 / 12.;

#[component]
pub fn BaseCardComponent<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(
    position: Vec2,
    width: f32,
    card: Option<C>,
    skin: S,

    // number_hint: Option<usize>,
    #[props(default = ColorMode::Light)]
    color_mode: ColorMode,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    ondoubleclick: EventHandler<MouseEvent>,
    #[props(default)]
    oncontextmenu: EventHandler<MouseEvent>,
) -> Element {
    let pt = width / 12.;
    let pt = |x: f32| {
        rem(x * pt)
    };

    if let Some(card) = &card {
        rsx! {
            div {
                style: "place-items: center;",
                position: "absolute",
                top: rem(position.y),
                left: rem(position.x),
                background_color: color_mode.choose("#fff", "#101"),
                width: pt(11.),
                height: pt(12.),
                border: "{pt(0.25)} solid",
                border_color: color_mode.choose("#000", "#868"),
                border_radius: rem(width * CARD_BORDER_RADIUS_RATIO),
                display: "grid",
                grid_template_columns: "50% 50%",
                grid_template_rows: "50% 50%",
                font_size: pt(5.),
                text_align: "center",
                padding: pt(0.25),
                color: skin.get_color(card, !color_mode),

                onclick, ondoubleclick, oncontextmenu,

                div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_rank(&card)}},
                div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_suit(&card)}},
                div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_suit(&card)}},
                div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_rank(&card)}},
            }
        }
    } else {
        rsx! {
            // div {
            //     style: "--card-width: {pt(11.)}",
            //     div {
            //         style: "place-items: center;",
            //         position: "absolute",
            //         top: rem(position.y),
            //         left: rem(position.x),
            //         background_color: "#fff",
            //         width: pt(11.),
            //         height: pt(12.),
            //         border: "{pt(0.25)} solid #000",
            //         border_radius: rem(width * CARD_BORDER_RADIUS_RATIO),
            //         padding: pt(0.25),
            //         display: "grid",
            //         onclick, ondoubleclick, oncontextmenu,

            //         div {
            //             class: "card-pattern-1",
            //             position: "relative",
            //             width: pt(10.75),
            //             height: pt(11.75),
            //             border_radius: pt(1.),
            //             display: "flex",
            //             justify_content: "center",
            //             align_items: "center",

            //             if let Some(number_hint) = number_hint {
            //                 div {
            //                     background: "rgba(192, 192, 192, 0.75)",
            //                     // position: "absolute",
            //                     // bottom: pt(0.5),
            //                     border_radius: pt(1.),
            //                     color: "#000",
            //                     font_family: KATEX_SUITS_FONT_STR,
            //                     font_size: pt(4.),
            //                     height: pt(4.5),
            //                     display: "flex",
            //                     align_items: "center",
            //                     padding: "{pt(0.25)} {pt(0.75)}",
            //                     "{number_hint}",
            //                 }
            //             }
            //         }
            //     }
            // }
            
        }
    }
}

pub const CARD_FRAME_DEFAULT_COLOR: &str = "#aaa";


#[component]
pub fn CardFrame(
    position: Vec2,
    width: f32,
    hint: Option<Element>,
    #[props(default = CARD_FRAME_DEFAULT_COLOR.to_string())] 
    color: String,
    onclick: EventHandler<MouseEvent>,
    // oncontextmenu: EventHandler<MouseEvent>,
    #[props(default)] dashed: bool,

    number_hint: Option<i32>,
) -> Element {
    let color = color.as_str();
    let pt = width / 12.;
    let pt = |x: f32| {
        rem(x * pt)
    };
    let border_style = if dashed {"dashed"} else {"solid"};
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            justify_content: "center",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            margin: pt(0.25), // frame must be slightly smaller than card to prevent peeking out in some platforms
            width: pt(10.),
            height: pt(11.),
            border: "{pt(0.5)} {border_style} {color}",
            text_align: "center",
            color,
            border_radius: pt(1.5),
            font_size: pt(5.),
            padding: pt(0.25),
            onclick, 
            //oncontextmenu,

            if let Some(hint) = hint {
                div {
                    {hint},
                }
            }

            if let Some(number_hint) = number_hint {
                div {
                    style: "place-items: center;",
                    position: "absolute",
                    left: pt(10.),
                    top: pt(-2.5),
                    height: "1.2em",
                    aspect_ratio: 1,
                    border_radius: "50%",
                    background_color: color,
                    color: "#000",
                    display: "grid",
                    font_size: "0.75em",
                    font_family: KATEX_SUITS_FONT_STR,

                    "{number_hint}"
                }
            }
        }
    }
}

#[component]
pub fn CardText<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(card: C, skin: S, color_mode: ColorMode) -> Element {
    rsx! {
        span {
            font_size: "1.2em",
            white_space: "nowrap",
            line_height: 1.2,
            color: skin.get_color(&card, color_mode),
            {skin.render_rank(&card)},
            span {display: "inline-block", min_width: "0.1em"},
            {skin.render_suit_text(&card)},
        }
    }
}

#[component]
pub fn CardComponent(
    position: Vec2,
    width: f32,
    card: Card,
    skin: Skin,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    ondoubleclick: EventHandler<MouseEvent>,
    #[props(default)]
    oncontextmenu: EventHandler<MouseEvent>,
) -> Element {
    let color_mode = if card.tapped {ColorMode::Dark} else {ColorMode::Light};
    rsx! {
        BaseCardComponent { 
            position, width, 
            card: Some(card), 
            skin,
            color_mode,
            onclick,
            ondoubleclick,
            oncontextmenu,
        }
    }
}