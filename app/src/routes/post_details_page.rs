use dioxus::prelude::*;

#[component]
pub fn PostDetails(id: String) -> Element {
    rsx! {
        h1 { "Post Details" }
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
        }
    }
}
