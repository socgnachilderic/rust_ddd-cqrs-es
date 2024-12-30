use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    models::CreatePostInput,
    servers::posts_server::{create_post_server, get_all_posts_server},
};

#[component]
pub fn Blog() -> Element {
    let mut posts_resources = use_server_future(get_all_posts_server)?;
    let data = posts_resources().unwrap().unwrap();
    let mut modalIsOpen = use_signal(|| false);

    let onsubmit = move |evt: FormEvent| async move {
        let title = &evt.values()["title"].as_value();
        let content = &evt.values()["content"].as_value();
        let post = CreatePostInput::new(title, content);

        let post_created = create_post_server(post).await.unwrap();
        info!("post: {post_created:?}");
        posts_resources.restart();
        modalIsOpen.set(false);
    };

    rsx! {
        main { class: "max-w-6xl mx-auto py-8",
            div { class: "flex justify-between items-center",
                h1 { class: "text-3xl", "Blog" }
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold h-9 px-4 text-sm inline-flex items-center rounded shadow",
                    onclick: move |_| { modalIsOpen.set(true) },
                    "New Post"
                }
            }

            if modalIsOpen() {
                div { class: "fixed inset-0 bg-black/50",
                    form {
                        class: [
                            "absolute  left-1/2 transform -translate-x-1/2 w-[400px] bg-white p-6 rounded-xl shadow-xl flex flex-col gap-4 transition-all duration-300 ease-in-out",
                            if !modalIsOpen() { "top-24 opacity-0" } else { "modal_open" },
                        ]
                            .join(" "),

                        onsubmit,
                        h3 { class: "text-2xl", "New Post" }

                        div { class: "space-y-2",
                            label {
                                r#for: "title",
                                class: "block text-gray-700 font-bold",
                                "Title *"
                            }
                            input {
                                id: "title",
                                name: "title",
                                required: true,
                                class: "border border-gray-300 rounded h-9 px-3 text-sm w-full",
                                placeholder: "Title",
                            }
                        }

                        div { class: "space-y-2",
                            label {
                                r#for: "content",
                                class: "block text-gray-700 font-bold",
                                "Content *"
                            }
                            textarea {
                                id: "content",
                                name: "content",
                                resize: "none",
                                height: "125px",
                                required: true,
                                class: "border border-gray-300 rounded p-3 text-sm w-full",
                                placeholder: "Content",
                            }
                        }

                        div { class: "flex justify-end gap-2",
                            button {
                                class: "self-end min-w-24 text-center bg-red-500 hover:bg-red-700 text-white font-bold h-9 px-4 text-sm inline-flex justify-center items-center rounded shadow",
                                onclick: move |_| { modalIsOpen.set(false) },
                                "Cancel"
                            }
                            button { class: "self-end min-w-24 text-center bg-blue-500 hover:bg-blue-700 text-white font-bold h-9 px-4 text-sm inline-flex justify-center items-center rounded shadow",
                                "Save"
                            }
                        }
                    }
                }
            }

            for post in data {
                div { class: "py-4",
                    h1 { class: "text-2xl", "Post: {post.title}" }
                    p { "{post.content}" }
                }
            }
        }
    }
}
