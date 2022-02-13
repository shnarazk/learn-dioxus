use {dioxus::prelude::*, std::collections::HashMap};
mod csv;

fn main() {
    dioxus::desktop::launch(App);
}

enum TableMode {
    Date,
    Location,
    Age,
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let csv = use_future(&cx, || async move { csv::load_csv().await });
    let (display_mode, set_display_mode) = use_state(&cx, || TableMode::Date);
    // let (_hash, update_hash) = use_state(&cx, HashMap::<&str, u32>::new);
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
            let mut dates: Vec<(&str, u32)> = ht_dates.iter().map(|(k, v)| (*k, *v)).collect();
            dates.sort_unstable();
            dates.reverse();
            dates.resize(21, ("", 0));
            let mut locs: Vec<(&str, u32)> = ht_locs
                .iter()
                .map(|(k, v)| (*k, *v))
                .filter(|(k, v)| !k.is_empty() && 100 <= *v)
                .collect();
            locs.sort_by_cached_key(|i| -(i.1 as i32));
            let table = match display_mode {
                TableMode::Age => rsx!(Table { data: ages }),
                TableMode::Date => rsx!(Table { data: dates }),
                TableMode::Location => rsx!(Table { data: locs }),
            };
            cx.render(rsx!(
                h1 {
                    style { [include_str!("../assets/main.scss")] }
                    "Fukuoka COVID-19 viewer: {len}"
                }
                button { onclick: move |_| {set_display_mode(TableMode::Age)}, "世代別" }
                button { onclick: move |_| {set_display_mode(TableMode::Date)}, "時間順" }
                button { onclick: move |_| {set_display_mode(TableMode::Location)}, "地区別" }
                table
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
