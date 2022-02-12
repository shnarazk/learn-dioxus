use {
    dioxus::{events::MouseEvent, prelude::*},
    std::collections::HashMap,
};
mod csv;

fn main() {
    dioxus::desktop::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let csv = use_future(&cx, || async move { csv::load_csv().await });
    // let (count, set_count) = use_state(&cx, || 0i32);
    let (_hash, update_hash) = use_state(&cx, HashMap::<&str, u32>::new);
    match csv.value() {
        Some(Ok(csv)) => {
            let len = csv.len();
            let mut locs: HashMap<&str, u32> = HashMap::new();
            let mut dates: HashMap<&str, u32> = HashMap::new();
            let mut ages: HashMap<&str, u32> = HashMap::new();
            for ci in csv.iter() {
                *dates.entry(&ci.date).or_insert(0) += 1;
                *locs.entry(&ci.location).or_insert(0) += 1;
                *ages.entry(&ci.age).or_insert(0) += 1;
            }
            let mut ages: Vec<(&str, u32)> = ages.iter().map(|(k, v)| (*k, *v)).collect();
            ages.sort_unstable();
            cx.render(rsx!(
                h1 {
                    style { [include_str!("../assets/main.scss")] }
                    "Fukuoka COVID-19 viewer: {len}"
                }
                button {
                    onclick: move |_| {}, "時間順"
                }
                button {
                    onclick: move |_| {}, "世代別"
                }
                button {
                    onclick: move |_| {}, "地区別"
                }
                // Quantity {
                //     on_up: move |_| set_count(count + 1),
                //     on_down: move |_| set_count(count - 1),
                // }
                // div {
                //     csv.iter().skip(len - 20).map(|l| rsx!(
                //         div {
                //             "{l:?}"
                //         }) )
                // }
                hr {}
                div {
                    style: "margin-left: 20px;margin-right: 20px; background-color: #eee;",
                    ages.iter().map(|a| rsx!(
                        div {
                            "{a:?}"
                        }
                    ))
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
