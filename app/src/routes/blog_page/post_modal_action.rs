use dioxus::prelude::*;
use lucide_dioxus::{Eye, Pencil};

use crate::models::Post;
use crate::{models::CreatePostInput, servers::posts_server::create_post_server};

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) enum PostModalAction {
    #[default]
    Create,
    Edit(Post),
    View(Post),
}

impl PostModalAction {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Create => "New Post",
            Self::Edit(_) => "Edit Post",
            Self::View(_) => "Post Details",
        }
    }

    pub fn post(&self) -> Post {
        match self {
            Self::Create => Post::default(),
            Self::Edit(post) => post.clone(),
            Self::View(post) => post.clone(),
        }
    }

    pub fn is_view(&self) -> bool {
        matches!(self, PostModalAction::View(_))
    }

    pub fn modal_action_button(&self, post: Post, mut action: Signal<PostModalAction>) -> Element {
        match self {
            Self::View(_) => rsx! {
                button {
                    class: "bg-gray-100 p-1.5 rounded-lg text-blue-500 hover:bg-blue-50",
                    onclick: move |_| {
                        action.set(Self::Edit(post.clone()));
                    },
                    Pencil { size: 20 }
                }
            },
            Self::Edit(_) => rsx! {
                button {
                    class: "bg-gray-100 p-1.5 rounded-lg text-blue-500 hover:bg-blue-50",
                    onclick: move |_| {
                        action.set(Self::View(post.clone()));
                    },
                    Eye { size: 20 }
                }
            },
            _ => rsx! {},
        }
    }

    pub async fn onsubmit(&self, title: &str, content: &str) {
        match self {
            Self::Create => {
                let post = CreatePostInput::new(title, content);
                create_post_server(post).await.unwrap();
            }
            Self::Edit(_) => {}
            Self::View(_) => {}
        }
    }
}
