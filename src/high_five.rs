use dioxus::{events::MouseEvent, prelude::*};

#[allow(non_snake_case)]
pub fn HighFiveApp(cx: Scope) -> Element {
    let count = use_state(&cx, || 0i32);
    cx.render(rsx!(
        h1 {
            style { [include_str!("../assets/main.scss")] }
            "High-Five counter: {count}"
        }
        Quantity {
            on_up: move |_| count.modify(|c| c + 1),
            on_down: move |_| count.modify(|c| c - 1),
        }
    ))
}

#[derive(Props)]
struct QuantityProps<'a> {
    on_up: EventHandler<'a, MouseEvent>,
    on_down: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
fn Quantity<'a>(cx: Scope<'a, QuantityProps<'a>>) -> Element<'a> {
    let button_style = "padding: 4px; background-color: #ccf; min-width: 60px;";
    cx.render(rsx!(
        button {
            style: "{button_style}",
            onclick: move |evt| cx.props.on_down.call(evt),
            "Down"
        }
        button {
            style: "{button_style}",
            onclick: move |evt| cx.props.on_up.call(evt),
            "Up"
        }
    ))
}
