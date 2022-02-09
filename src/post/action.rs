use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct ActionCardProps{
    post_id: u32,
}

#[allow(non_snake_case)]
pub fn ActionCard(cx: Scope<ActionCardProps>) -> Element {
    cx.render(rsx!{
        div {
            "share"
        }
    })
}
