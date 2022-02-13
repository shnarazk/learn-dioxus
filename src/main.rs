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
            let mut ht_locs: HashMap<&str, u32> = HashMap::new();
            let mut ht_dates: HashMap<&str, u32> = HashMap::new();
            let mut ht_ages: HashMap<&str, u32> = HashMap::new();
            for ci in csv.iter() {
                *ht_dates.entry(&ci.date).or_insert(0) += 1;
                *ht_locs.entry(&ci.location).or_insert(0) += 1;
                *ht_ages.entry(&ci.age).or_insert(0) += 1;
            }
            let mut ages: Vec<(&str, u32)> = ht_ages
                .iter()
                .map(|(k, v)| (*k, *v))
                .filter(|(k, _)| !k.is_empty())
                .collect();
            ages.sort_unstable();
            let mut locs: Vec<(&str, u32)> = ht_locs
                .iter()
                .map(|(k, v)| (*k, *v))
                .filter(|(k, v)| !k.is_empty() && 100 <= *v)
                .collect();
            locs.sort_by_cached_key(|i| -(i.1 as i32));
            cx.render(rsx!(
                h1 {
                    style { [include_str!("../assets/main.scss")] }
                    "Fukuoka COVID-19 viewer: {len}"
                }
                button { onclick: move |_| {}, "時間順" }
                button { onclick: move |_| {}, "世代別" }
                button { onclick: move |_| {}, "地区別" }
                Table { data: ages }
                Table { data: locs }
            ))
        }
        _ => cx.render(rsx!("Fetching data ...")),
    }
}

#[derive(Default, PartialEq, PartialOrd, Props)]
struct TableProps<'a> {
    data: Vec<(&'a str, u32)>,
}

#[allow(non_snake_case)]
fn Table<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {
    cx.render(rsx!(
                hr {}
                div {
                    style: "margin-left: 20px;margin-right: 20px; background-color: #eee;",
                    class: "table",
                    cx.props.data.iter().enumerate().map(|(i, (k, v))| {
                        let style = if i % 2 == 0 {
                            "background-color: #eeeeee;"
                        } else {
                            "background-color: #eaeaea;"
                        };
                        rsx!(
                            div {
                                style: "{style}",
                                div {
                                    style: "display: inline-block; width: 180px; margin-left: 20px; text-align: left;",
                                    "{k}"
                                }
                                div {
                                    style: "display: inline-block; width: 120px; margin-left: 10px; text-align: right;",
                                    "{v}"
                                }
                            }
                        )
                    })
                }
    ))
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
