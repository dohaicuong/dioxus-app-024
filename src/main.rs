#![allow(non_snake_case)]

use dioxus::prelude::*;

mod atoms;
mod pages;
use pages::{
    home::HomePage,
    other::OtherPage
};

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app); 
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        Router {
            ul {
                Link { to: "/", li { "home" } }
                Link { to: "/other", li { "other" } }
                Link { to: "/broken", li { "broken" } }
            }
            Route { to: "/", HomePage {} },
            Route { to: "/other", OtherPage {} },
            Redirect { from: "", to: "/" }
        }
    ))
}
