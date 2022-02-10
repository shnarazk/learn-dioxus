use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct VoteButtonProps {}

#[allow(non_snake_case)]
pub fn VoteButton(cx: Scope<VoteButtonProps>) -> Element {
    cx.render(rsx! {
        div {
            "UpDown"
        }
    })
}
