#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
    log("Hello from Rust!");
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
    let mut count: Signal<i32> = use_signal(|| 0);

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
                button { onclick: move |_| {
                    match count().checked_add(1) {
                        Some(new_count) => {
                            count.set(new_count);
                            log(&format!("Count increased to {}", new_count));
                        },
                        None => log("Error: Overflow detected when incrementing the count"),
                    }
                }, "Up high!" }
                button { onclick: move |_| {
                    match count().checked_sub(1) {
                        Some(new_count) => {
                            count.set(new_count);
                            log(&format!("Count decreased to {}", new_count));
                        },
                        None => log("Error: Overflow detected when decrementing the count"),
                    }
                }, "Down low!" }
                button { onclick: move |_| {
                    match count().checked_mul(2) {
                        Some(new_count) => {
                            count.set(new_count);
                            log(&format!("Count doubled to {}", new_count));
                        },
                        None => log("Error: Overflow detected when doubling the count"),
                    }
                }, "Double!" }
                button { onclick: move |_| count.set(0), "Reset!" }
            }
        }
    }
}
