#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;


#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        link { rel: "stylesheet", href: "main.css" }
        div {
            h1 { "High-Five counter: {count}" }
            div { "i32_max = {std::i32::MAX}" }
            div { "u32_max = {std::u32::MAX}" }
            div { "i128_max = {std::i128::MAX}" }
            div { "u128_max = {std::u128::MAX}" }
            div {
                class: "setButton",
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }
                button { onclick: move |_| count *= 2, "Double!" }
                button { onclick: move |_| count.set(0), "Reset!" }
            }
        }
    }
}
