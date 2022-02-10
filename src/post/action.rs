use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct ActionCardProps {
    post_id: u32,
}

#[allow(non_snake_case)]
pub fn ActionCard(cx: Scope<ActionCardProps>) -> Element {
    cx.render(rsx! {
        div {
            style: "display: flex; flex: auto; margin: 8px; background-color: #ccf;",
            span {
                style: "margin: 8px;",
                "ActionCard"
            }
            span {
                style: "margin: 8px;",
                "share"
            }
            span {
                style: "margin: 8px;",
                "save"
            }
            span {
                style: "margin: 8px;",
                "hide"
            }
            span {
                style: "margin: 8px;",
                "award"
            }
        }
    })
}
