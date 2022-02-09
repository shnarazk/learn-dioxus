use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct PostProps {
    post_id: u32,
}

pub fn Post(cx: Scope<PostProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "post-container",
            action::ActionCard {
                post_id: cx.props.post_id
            }
        }
    })
}

pub mod action;
pub use action::ActionCard;
