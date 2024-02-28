use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use crate::DataProvider;

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
    fn get_name(&self, data: &DataProvider) -> &str;
    fn get_link(&self, data: &DataProvider) -> String;
    fn main_body(&self, data: &DataProvider) -> Element;


    fn main_card(&self, data: &DataProvider) -> Element;
}

