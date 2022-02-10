use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct PostMetaProps {}

#[allow(non_snake_case)]
pub fn PostMeta(cx: Scope<PostMetaProps>) -> Element {
    cx.render(rsx! {
        div {
            "META for original poster, time submitted"
        }
    })
}
