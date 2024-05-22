use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct Campaign{
    pub name: String,
    pub total_price: f64,
    pub type_crypto: String,
    pub url: String,
    pub remuneration: f64
}