use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct ActionCardProps{
    post_id: u32,
}

pub fn ActionCard(cx: Scope<ActionCardProps>) -> Element {
    cx.render(rsx!{
        div {
            "share"
        }
    })
}
