use dioxus::prelude::*;

use crate::Route;

const LOGO: Asset = asset!("/assets/favicon.ico",);

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "bg-neutral-800 text-white",
            div { class: "flex items-center justify-between max-w-6xl mx-auto py-2",
                div { class: "flex items-center space-x-5",
                    img { src: LOGO, class: "size-10" }
                    h3 { class: "text-lg", "Todo App" }
                }

                nav { class: "space-x-1 text-center",
                    Link {
                        to: Route::Home {},
                        active_class: "bg-neutral-900",
                        class: "hover:bg-neutral-700 px-4 py-3 inline-block min-w-24",
                        "Home"
                    }
                    Link {
                        to: Route::Blog {},
                        active_class: "bg-neutral-900",
                        class: "hover:bg-neutral-700 px-4 py-3 inline-block min-w-24",
                        "Blog"
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
