#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::list::List;
use crate::components::new_campaign::NewCampaign;



use crate::context;

#[component]
pub fn Content() -> Element {
    // let mut is_loading = use_signal(|| false);
    let menu_selected = use_context::<Signal<context::Menu>>();
        
    rsx!{
        div{
            class: " h-full w-full ",
            match menu_selected().select.as_str() {
                "all-campaign"=> rsx! {List{}},
                "new-campaign"=> rsx! { 
                    NewCampaign{}
                },
                _ => rsx! { NewCampaign{} }
            }
        }
    }
}
