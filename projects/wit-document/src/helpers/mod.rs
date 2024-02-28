use crate::DataProvider;
use dioxus::{core_macro::rsx, dioxus_core::Element};
use std::borrow::Cow;

pub trait DocumentElementIcon {
    fn get_icon_name(&self) -> char;
    fn get_icon_color(&self) -> String {
        "black".to_string()
    }
    fn get_text_color(&self) -> String {
        "black".to_string()
    }
}

#[allow(unused_variables)]
pub trait DocumentElement {
    fn get_name(&self, data: &DataProvider) -> Cow<str>;
    fn get_link(&self, data: &DataProvider) -> String;
    fn main_body(&self, data: &DataProvider) -> Element;

    fn main_card(&self, data: &DataProvider) -> Element;
}
