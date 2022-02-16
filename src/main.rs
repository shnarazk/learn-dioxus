use {dioxus::prelude::*, std::collections::HashMap};
mod csv;

fn main() {
    dioxus::desktop::launch(App);
}

#[derive(PartialEq)]
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
            ages.sort_by_cached_key(|(e, _)| {
                if *e == "10歳未満" {
                    return "10代".to_string();
                }
                let mut s = e.to_string();
                if s.chars().count() == 3 {
                    s.push('_');
                }
                s
            });
            // ages.sort_unstable();
            let mut dates: Vec<(&str, u32)> = ht_dates.iter().map(|(k, v)| (*k, *v)).collect();
            dates.sort_unstable();
            dates = dates
                .iter()
                .skip(dates.len().saturating_sub(50))
                .copied()
                .collect::<Vec<_>>();
            let mut locs: Vec<(&str, u32)> = ht_locs
                .iter()
                .map(|(k, v)| (*k, *v))
                .filter(|(k, v)| !k.is_empty() && 100 <= *v)
                .collect();
            locs.sort_by_cached_key(|i| -(i.1 as i32));
            let table = match display_mode {
                TableMode::Age => rsx!(Table {
                    data: ages,
                    with_ema: false
                }),
                TableMode::Date => rsx!(Table {
                    data: dates,
                    with_ema: true
                }),
                TableMode::Location => rsx!(Table {
                    data: locs,
                    with_ema: false
                }),
            };
            let button_age = if *display_mode == TableMode::Age {
                rsx!(
                    button {
                        class: "current-mode",
                        onclick: move |_| {set_display_mode(TableMode::Age)},
                        "世代別"
                    }
                )
            } else {
                rsx!(
                    button {
                        class: "other-mode",
                        onclick: move |_| {set_display_mode(TableMode::Age)},
                        "世代別"
                    }
                )
            };
            let button_date = if *display_mode == TableMode::Date {
                rsx!(
                    button {
                        class: "current-mode",
                        onclick: move |_| {set_display_mode(TableMode::Date)},
                "時間順"
                    }
                )
            } else {
                rsx!(
                    button {
                        class: "other-mode",
                        onclick: move |_| {set_display_mode(TableMode::Date)},
                        "時間順"
                    }
                )
            };
            let button_loc = if *display_mode == TableMode::Location {
                rsx!(
                    button {
                        class: "current-mode",
                        onclick: move |_| {set_display_mode(TableMode::Location)},
                        "地区別"
                    }
                )
            } else {
                rsx!(
                    button {
                        class: "other-mode",
                        onclick: move |_| {set_display_mode(TableMode::Location)},
                        "地区別"
                    }
                )
            };
            cx.render(rsx!(
                h1 {
                    style { [include_str!("../assets/main.scss")] }
                    "Fukuoka COVID-19 viewer: {len}"
                }
                button_age
                button_date
                button_loc
                table
            ))
        }
        _ => cx.render(rsx!("Fetching data ...")),
    }
}

#[derive(Default, PartialEq, PartialOrd, Props)]
struct TableProps<'a> {
    data: Vec<(&'a str, u32)>,
    with_ema: bool,
}

#[allow(non_snake_case)]
fn Table<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {
    let graph_width: f32 = 400.0;
    let graph_height: f32 = 100.0;
    let height: f32 = cx
        .props
        .data
        .iter()
        .map(|e| (e.1 / 2000 + 1) * 2000)
        .max()
        .unwrap() as f32;
    let width: f32 = cx.props.data.len() as f32;
    let scale_w = graph_width / (width - 1.0);
    let scale_h = graph_height / height;
    let path = format!(
        "M0,{:.2} {}",
        graph_height - (cx.props.data[0].1 as f32) * scale_h,
        cx.props
            .data
            .iter()
            .enumerate()
            .map(|(i, (_, v))| {
                format!(
                    "L{:.2},{:.2}",
                    i as f32 * scale_w,
                    graph_height - *v as f32 * scale_h
                )
            })
            .collect::<Vec<_>>()
            .join(" "),
    );
    let mut value_vec: Vec<f32> = cx
        .props
        .data
        .iter()
        .map(|(_, v)| *v as f32)
        .clone()
        .collect::<Vec<_>>();
    let line_ema = if cx.props.with_ema {
        let days: f32 = 7.0;
        let mut ema: f32 = value_vec[0];
        format!(
            "M0,{:.2} {}",
            graph_height - value_vec[0] * scale_h,
            value_vec
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    ema *= (days - 1.0) / days;
                    ema += v / days;
                    format!(
                        "L{:.2},{:.2}",
                        i as f32 * scale_w,
                        graph_height - ema * scale_h
                    )
                })
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        "".to_string()
    };
    {
        let first = cx.props.data[0].1 as f32;
        for _ in 0..6 {
            value_vec.insert(0, first);
        }
    }
    let line_average = if cx.props.with_ema {
        let average = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;
        format!(
            "M0,{:.2} {}",
            graph_height - value_vec[0] * scale_h,
            value_vec
                .windows(7)
                .enumerate()
                .map(|(i, v)| {
                    format!(
                        "L{:.2},{:.2}",
                        i as f32 * scale_w,
                        graph_height - average(v) * scale_h
                    )
                })
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        "".to_string()
    };
    let cell_style = "display: inline-block; width: 180px; margin-left: 20px; text-align: left;";
    cx.render(rsx!(
        hr {}
        div {
            style: "width: 94%; margin-left: 3%; margin-bottom: 1rem; background-color: #f8f8f8;",
            svg {
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",

                view_box: "0 0 400 100",
                path {
                    stroke: "red",
                    stroke_width: "0.8",
                    d: "{line_average}"
                }
                path {
                    stroke: "green",
                    stroke_width: "0.4",
                    stroke_dasharray: "6 2",
                    d: "{line_ema}"
                }
                path {
                    stroke: "currentColor",
                    stroke_width: "1",
                    d: "{path}"
                }
            }
        }
        div {
            style: "margin-left: 20px;margin-right: 20px; background-color: #eee; height: 280px; overflow: scroll;",
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
                            style: "{cell_style}",
                            "{k}"
                        }
                        div {
                            style: "{cell_style}",
                            class: "right-aligned",
                            "{v}"
                        }
                    }
                )
            })
        }
    ))
}
