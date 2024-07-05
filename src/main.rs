use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        div {
            "Hello, world!"
        }
    }
}
