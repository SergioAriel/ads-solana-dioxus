use std::ptr::null;

use dioxus::prelude::*;

use crate::context::Campaign;

pub fn NewCampaign() -> Element {
    let mut campaign:Signal<Campaign> = use_signal(|| Campaign{
        name: "".to_string(),
         total_price: 0.0,
         type_crypto: "String".to_string(),
         url: "String".to_string(),
         remuneration: 0.0
    });
    let mut campaigns: Signal<Vec<Campaign>> = use_context::<Signal<Vec<Campaign>>>();
    let cssInput = "shadow-sm shadow-slate-800 w-full";

    rsx! {
        div {
            class:"w-full h-full flex flex-col items-center p-5",

            h1{ "Name Campaign"}


                div{
                    class: " w-full flex flex-col items-center",
                    label { "name" }
                    input { class:cssInput, name: "name" }
                }
                div{
                    class: " w-full flex flex-col items-center",
                    label { "total_cost" }
                    input { class:cssInput, name: "total_cost", r#type: "number" }
                }
                div{
                    class: " w-full flex flex-col items-center",
                    label { "crypto" }
                    input { class:cssInput, name: "crypto" }
                }
                div{
                    class: " w-full flex flex-col items-center",
                    label { "remuneration" }
                    input { class:cssInput, name: "remuneration" }
                }
                div{
                    class: " w-full flex flex-col items-center",
                    label { "url" }
                    input { class:cssInput, name: "url",
                    oninput: { move |_|
                        {}
                    }
                }
                }

                // button{
                //     onclick: {move |_| campaigns.push(Campaign{
                //             // name: event.values().name,
                //             // total_price: event.values().total_price,
                //             // type_crypto: event.values().type_crypto,
                //             // url: event.values().url,
                //             // remuneration: event.values().remuneration
                //         })
                //     }
                // }

        }
    }
}
