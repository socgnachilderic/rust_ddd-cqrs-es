use dioxus::prelude::*;
use lucide_dioxus::X as CloseIcon;

use super::post_modal_action::PostModalAction;

#[component]
pub(super) fn PostModalForm(
    is_open: Signal<bool>,
    is_loading: Signal<bool>,
    action: Signal<PostModalAction>,
    onsubmit: EventHandler<FormEvent>,
    onclose: Callback,
) -> Element {
    let post = use_memo(move || action().post());
    let title = use_memo(move || action().title());

    if !is_open() {
        return rsx! {};
    }

    rsx! {
        div { class: "fixed inset-0 bg-black/50",
            form {
                class: [
                    "absolute  left-1/2 transform -translate-x-1/2 w-[400px] bg-white p-6 rounded-xl shadow-xl flex flex-col gap-4 transition-all duration-300 ease-in-out",
                    if !is_open() { "top-24 opacity-0" } else { "modal_open" },
                ]
                    .join(" "),

                onsubmit,
                div { class: "flex justify-between items-start gap-2 ",
                    h3 { class: "text-2xl", "{title}" }
                    div { class: "space-x-2 -mr-3 -mt-2",
                        {action().modal_action_button(post(), action.clone())}

                        button {
                            class: "bg-gray-100 p-1.5 rounded-lg text-red-500 hover:bg-red-50",
                            onclick: move |_| {
                                onclose(());
                            },
                            CloseIcon { size: 20 }
                        }
                    }
                }


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
                        placeholder: "Title",
                        value: "{post().title}",
                        disabled: action().is_view() || is_loading(),
                        class: "border border-gray-300 rounded h-9 px-3 text-sm w-full",
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
                        placeholder: "Content",
                        value: "{post().content}",
                        disabled: action().is_view() || is_loading(),
                        class: "border border-gray-300 rounded p-3 text-sm w-full",
                    }
                }

                if !action().is_view() {
                    div { class: "flex justify-end gap-2",
                        button {
                            class: "self-end min-w-24 text-center bg-blue-500 hover:bg-blue-700 text-white font-bold h-9 px-4 text-sm inline-flex justify-center items-center rounded shadow",
                            disabled: is_loading(),
                            "Save"
                        }
                    }
                }
            }
        }
    }
}
