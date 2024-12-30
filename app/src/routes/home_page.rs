use dioxus::prelude::*;

use crate::components::{atoms::Echo, organisms::Hero};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
        Echo {}
    }
}
