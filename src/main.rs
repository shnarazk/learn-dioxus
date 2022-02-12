use dioxus::{events::MouseEvent, prelude::*};
mod csv;

fn main() {
    dioxus::desktop::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let csv = use_future(&cx, || async move { csv::load_csv().await });
    let (count, set_count) = use_state(&cx, || 0i32);
    match csv.value() {
        Some(Ok(csv)) => {
            let len = csv.len();
            cx.render(rsx!(
                h1 {
                    style { [include_str!("../assets/main.scss")] }
                    "Fukuoka COVID-19 viewer: {len}"
                }
                Quantity {
                    on_up: move |_| set_count(count + 1),
                    on_down: move |_| set_count(count - 1),
                }
                div {
                    csv.iter().skip(len - 20).map(|l| rsx!(
                        div {
                            "{l}"
                        }) )
                }
            ))
        }
        _ => cx.render(rsx!("Fetching data ...")),
    }
}

#[derive(Default, PartialEq, Props)]
struct CovidProps {
    csv: Vec<String>,
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
