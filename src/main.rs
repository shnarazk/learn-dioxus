use dioxus::prelude::*;

mod post;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    // let mut state = use_state(&cx, || "red");
    cx.render(rsx! (
        post::ActionCard {
            post_id: 1,
        }
    ))
}

// #[derive(PartialEq, Props)]
// struct Props{}
// 
// fn Post(_: Scope<Props>) -> Element { todo!() }
