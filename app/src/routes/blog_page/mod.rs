mod post_card_list;
mod post_modal_action;
mod post_modal_form;

use dioxus::prelude::*;
use post_card_list::PostCardList;
use post_modal_action::PostModalAction;
use post_modal_form::PostModalForm;

use crate::{models::Post, servers::posts_server::get_all_posts_server};

#[component]
pub fn Blog() -> Element {
    let mut posts_resources = use_server_future(get_all_posts_server)?;
    let data = posts_resources().unwrap().unwrap();
    let mut modal_action = use_signal(|| PostModalAction::default());
    let mut modal_is_open = use_signal(|| false);
    let mut modal_is_loading = use_signal(|| false);

    let onclose = move || async move {
        modal_is_open.set(false);
    };

    let onsubmit = move |evt: FormEvent| async move {
        let title = &evt.values()["title"].as_value();
        let content = &evt.values()["content"].as_value();

        modal_is_loading.set(true);
        modal_action().onsubmit(title, content).await;
        posts_resources.restart();
        modal_is_loading.set(false);
        onclose().await;
    };

    rsx! {
        main { class: "max-w-6xl mx-auto py-8",
            div { class: "flex justify-between items-center",
                h1 { class: "text-3xl", "Blog" }
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold h-9 px-4 text-sm inline-flex items-center rounded shadow",
                    onclick: move |_| {
                        modal_is_open.set(true);
                        modal_action.set(PostModalAction::Create);
                    },
                    "New Post"
                }
            }

            PostModalForm {
                is_loading: modal_is_loading,
                is_open: modal_is_open,
                action: modal_action,
                onsubmit,
                onclose,
            }

            PostCardList {
                items: data,
                onviewpost: Callback::new(move |post: Post| {
                    modal_is_open.set(true);
                    modal_action.set(PostModalAction::View(post));
                }),
            }
        }
    }
}
