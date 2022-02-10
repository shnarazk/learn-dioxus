use dioxus::prelude::*;

mod action;

#[derive(PartialEq, Props)]
pub struct PostProps {
    post_id: u32,
    score: i32,
    comment_count: u32,
    post_time: std::time::Instant,
    url: String,
    title: String,
    original_poster: String,
}

#[allow(unused)]
#[allow(non_snake_case)]
pub fn Post(cx: Scope<PostProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "post-container",
            action::ActionCard {
                post_id: cx.props.post_id
            }
            ul {
                li {
                    "{cx.props.score}"
                }
            }
        }
    })
}

pub use action::ActionCard;
