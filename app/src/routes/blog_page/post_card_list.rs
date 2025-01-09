use dioxus::prelude::*;
use lucide_dioxus::{Eye, Trash};

use crate::models::Post;

#[component]
pub fn PostCardList(items: Vec<Post>, mut onviewpost: Callback<Post>) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 lg:gap-8 mt-8",
            for item in items {
                PostCard { post: item, onviewpost }
            }
        }
    }
}

#[component]
fn PostCard(post: Post, mut onviewpost: Callback<Post>) -> Element {
    rsx! {
        div { class: "py-4 px-6 bg-white rounded-lg shadow-md border border-gray-200 text-gray-600",
            div { class: "flex justify-between items-center",
                h1 { class: "text-2xl capitalize", "{post.title}" }
                div { class: "flex justify-end items-center gap-2 -mr-3 -mt-1",
                    button {
                        class: "bg-gray-100 p-1.5 rounded-lg text-blue-500 hover:bg-blue-50",
                        onclick: move |_| {
                            onviewpost.call(post.clone());
                        },
                        Eye { size: 20 }
                    }
                    button { class: "bg-gray-100 p-1.5 rounded-lg text-red-500 hover:bg-red-50",
                        Trash { size: 20 }
                    }
                }
            
            }
            p { class: "min-h-[100px]", "{post.content}" }
        }
    }
}
