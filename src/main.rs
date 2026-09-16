use dioxus::prelude::*;

use crate::components::Hero;

mod game;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");

// altered version of KaTeX_Main to include filled "red" suits
const KATEX_SUITS: Asset = asset!("/assets/KaTeX_Suits.woff2");

// from https://www.confettijs.org/
const CONFETTI_JS: Asset = asset!("/assets/confetti.min.js");

const STATIC_CSS: bool = !cfg!(debug_assertions);

const MAIN_CSS: Asset = asset!("/assets/main.css");
const MAIN_CSS_STR: &str = const_css_minify::minify!("../assets/main.css");
const _: &str = include_str!("../assets/main.css"); // ensures recompilation if CSS changed

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.min.css",
            integrity: "sha384-zh0CIslj+VczCZtlzBcjt5ppRcsAmDnRem7ESsYwWwg3m/OaJ2l4x7YBZl9Kxxib",
            crossorigin: "anonymous"
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Noto+Emoji:wght@300..700&family=Noto+Sans+Math&family=Noto+Sans+Symbols+2&family=Noto+Sans+Symbols:wght@100..900&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
        }
        document::Link { rel: "icon", href: FAVICON }

        document::Style {
            r#"
            @font-face {{
                font-family: KaTeX_Suits;
                font-style: normal;
                font-weight: 700;
                src: url({KATEX_SUITS}) format("woff2");
            }} 
            "#,
        }

        if STATIC_CSS {
            document::Style { {MAIN_CSS_STR} }
        } else {
            // visibility hidden to prevent FOUC, is set back to visible in MAIN_CSS
            document::Style {r#"
                html {{
                    visibility: hidden;
                }}
            "#,}
            document::Link { href: MAIN_CSS, rel: "stylesheet" }
        }

        document::Script { src: CONFETTI_JS }
        Hero {}

    }
}
