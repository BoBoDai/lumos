use dioxus::prelude::*;

static CSS: Asset = asset!("/src/components/hero.module.css");

#[component]
pub fn Hero() -> Element {
    rsx! {
    document::Link { rel: "stylesheet", href: CSS }
        div {
            class: "font",
            "dog-container"
        }
    }
}
