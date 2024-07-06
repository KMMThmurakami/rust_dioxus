use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render! {
        h1 { "Counter: {count}" }

        button {
            onclick: move |_| {
                count += 1
            },
            "+"
        }

        button {
            onclick: move |_| {
                count -= 1
            },
            "-"
        }
    }
}
