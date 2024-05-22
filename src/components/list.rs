use dioxus::prelude::*;
use crate::context::Campaign;
use crate::components::cards::Cards;



// use dioxus_logger::tracing::{info};

// macro_rules! vec_of_strings {
//     ($($x:expr),*) => (vec![$($x.to_string()),*]);
// }

pub fn List() -> Element {
    // let mut array_test = use_signal(|| vec_of_strings![ "ToDo1", "ToDo2", "ToDo3", "ToDo3", "ToDo4"]);
    let campaigns: Signal<Vec<Campaign>> = use_context::<Signal<Vec<Campaign>>>();

    rsx! {
        div {
            class: "w-full h-full border-b flex flex-wrap",
            for campaign in campaigns.iter() {
                Cards{
                    name: campaign.name.clone(),
                    total_price: campaign.total_price.clone(),
                    type_crypto: campaign.type_crypto.clone(),
                    url: campaign.url.clone(),
                    remuneration: campaign.remuneration.clone()
                }
            }
            
        }
    }
}