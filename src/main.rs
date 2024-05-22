#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
// use std::thread::Scope;

use dioxus::prelude::*;
use tracing::Level;

mod api;
mod components;
mod context;
mod pages;

// use api::service_solana;
use pages::home::Content;

use crate::context::Campaign;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    message: String,
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Wrappert)]
    #[route("/")]
    Content {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| {
        Signal::new(Vec::<Campaign>::new())
    });
    use_context_provider(|| 
        Signal::new(context::Menu{
            select: "new-campaign".to_string()
        })
    );
    rsx! {
        Router::<Route> {}
    }
}

fn Wrappert() -> Element {


    rsx! {
        div{
            class: "w-screen h-screen",
            div { 
                class: "w-full h-16 p-4 flex flex-col justify-between border-b",
                p {
                    class: "text-xl font-bold ",
                    "ADS-Solana"
                }
            }
            div {
                class: "w-full flex h-[calc(100%_-_4rem)]",
                components::menu::Menu{}
                Outlet::<Route> {}
            }
        }
    }
}
