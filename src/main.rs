use dioxus::prelude::*;


fn main() {
    dioxus::desktop::launch(App);
}

fn App(cx: Scope) -> Element {

let mut state = use_state(&cx, || "red");
    // cx.render(rsx! (
    //     div { "Hello, world!" }
    // ))
    cx.render(rsx!(
        Container {
            Light { color: "red", enabled: state == "red", }
            Light { color: "yellow", enabled: state == "yellow", }
            Light { color: "green", enabled: state == "green", }

            onclick: move |_| {
                state.set(match *state {
                    "green" => "yellow",
                    "yellow" => "red",
                    "red" => "green",
                })
            }
        }
    ))
}
