use dioxus::prelude::*;

use components::Hero;

mod components;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}
