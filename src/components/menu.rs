use dioxus::prelude::*;

use crate::context::Menu;

pub fn Menu() -> Element {
    let mut select_menu: Signal<Menu> = use_context::<Signal<Menu>>();

    rsx! {
        div {
            class: "w-46 border-r flex flex-col gap-2",
            button {
                onclick: move |_| {
                    select_menu.set(Menu{
                        select:"new-campaign".to_string()
                    })
                },
                "New Campaign"
            }
            button {
                onclick: move |_| {
                    select_menu.set(Menu{
                        select:"all-campaign".to_string()
                    })
                },
                "All Campaign"
            }
        }
    }
}