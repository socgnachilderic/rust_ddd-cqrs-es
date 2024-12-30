mod blog_page;
mod home_page;
mod not_found_page;
mod post_details_page;

use blog_page::Blog;
use dioxus::prelude::*;
use home_page::Home;
use not_found_page::NotFound;
use post_details_page::PostDetails;

use crate::components::organisms::Navbar;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[nest("/blog")]
        #[route("/")]
        Blog { },
        #[route("/post/:id")]
        PostDetails { id: String },
    #[end_nest]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
