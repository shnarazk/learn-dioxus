use dioxus::prelude::*;

mod post;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    // let mut state = use_state(&cx, || "red");
    cx.render(rsx! (
        post::Post {
            post_id: 1,
            score: 10,
            comment_count: 10,
            post_time: std::time::Instant::now(),
            url: "example".to_string(),
            title: "Title".to_string(),
            original_poster: "me".to_string(),
        }
    ))
}
