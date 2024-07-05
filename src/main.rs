use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        h1 { "Title" }

        div {
            "Hello, world!"
            div {
                "inner"
            }
        }

        br {}    
    }
}
