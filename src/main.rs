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

#[derive(Debug)]
enum CalcFlag {
    Add,
    Sub,
    Multiple,
    Max,
    Min,
    Reset
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct CalcStatus( bool );

#[component]
fn Home() -> Element {
    let mut count: Signal<i32> = use_signal(|| 0);
    let mut state: Signal<CalcStatus> = use_signal(|| CalcStatus(true));

    let mut update_count = move |flg: CalcFlag| {
        let new_count: Option<i32>;
        match flg {
            CalcFlag::Add => new_count = count().checked_add(1),
            CalcFlag::Sub => new_count = count().checked_sub(1),
            CalcFlag::Multiple => new_count = count().checked_mul(2),
            CalcFlag::Max => new_count = Some(std::i32::MAX),
            CalcFlag::Min => new_count = Some(std::i32::MIN),
            CalcFlag::Reset => new_count = Some(0),
        }
        match new_count {
            Some(new_value) => {
                count.set(new_value);
                state.set(CalcStatus(true));
                log(&format!("Count changed to {}", new_value));
            },
            None => {
                state.set(CalcStatus(false));
                log("Error: Overflow detected when changing the count");
            },
        }
    };

    let loop_max = 10000;

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
            div {
                class: "statusArea",
                if state() == CalcStatus(false) {
                    div { "Overflow i32!!" }
                }
            }
            div { class: "integerText", "i32"
                div { "max = {std::i32::MAX}" }
                div { "min = {std::i32::MIN}" }
            }
            div {
                class: "setButton",
                button { onclick: move |_| update_count(CalcFlag::Add), "+1" }
                button { onclick: move |_| update_count(CalcFlag::Sub), "-1" }
                button { onclick: move |_| update_count(CalcFlag::Multiple), "×2" }
                button { onclick: move |_| {
                    let mut i = 0;
                    while i < loop_max {
                        update_count(CalcFlag::Add);
                        i += 1;
                    }
                }, "loop {loop_max}" }
                button { onclick: move |_| update_count(CalcFlag::Max), "MAX!" }
                button { onclick: move |_| update_count(CalcFlag::Min), "MIN!" }
                button { onclick: move |_| update_count(CalcFlag::Reset), "Reset!" }
            }
        }
    }
}
