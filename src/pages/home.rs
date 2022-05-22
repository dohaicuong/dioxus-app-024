use dioxus::{prelude::*, fermi::use_atom_state};
use crate::atoms::title::TITLE;

pub fn HomePage(cx: Scope) -> Element {
    let title = use_atom_state(&cx, TITLE);

    cx.render(rsx! {
        h1 { "Home page" }
        p { "Title atom: {title}" }
    })
}
