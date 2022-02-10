use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct TitleCardProps {
    title: String,
}

#[allow(non_snake_case)]
pub fn TitleCard(cx: Scope<TitleCardProps>) -> Element {
    cx.render(rsx! {
        div {
            "Title: {cx.props.title}"
        }
    })
}
