use crate::DataProvider;
use dioxus::{html::s, prelude::*};
use wit_parser::{Interface, TypeDef, TypeDefKind, World};

mod render_types;
mod render_interface;
mod render_world;

pub fn render_interface(data: &DataProvider, interface: &Interface) -> Element {
    let words = data.package.worlds.iter().map(|(key, value)| value.left_link());
    let interfaces = data.package.interfaces.iter().map(|(key, value)| value.left_link());
    let card = interface.main_card();

    rsx! {
        div { class: "container",
            div { class: "lift-list", {words}, {interfaces} }
            div { class: "left-separator" }
            ul { class: "main-list" }
        }
    }
}


#[allow(unused_variables)]
pub trait DocumentElement {
    fn left_link(&self) -> Element;

    fn main_body(&self, data: &DataProvider) {}

    fn main_card(&self, data: &DataProvider) -> Element;
    fn main_link(&self) -> Element;
}
