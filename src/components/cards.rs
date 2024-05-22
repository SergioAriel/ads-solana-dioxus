use dioxus::prelude::*;
use crate::context::Campaign;

pub fn Cards(props: Campaign) -> Element {
    rsx! {
        div {

            class: "h-40 w-56 border border-red flex items-center",
            p {
                "{props.name}"
            }
            p {
                "{props.total_price}"
            }
            p {
                "{props.type_crypto}"
            }
        }
    }
}
