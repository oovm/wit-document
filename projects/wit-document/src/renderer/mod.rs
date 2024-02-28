use crate::DataProvider;
use dioxus::{html::s, prelude::*};
use wit_parser::{Interface, TypeDef, TypeDefKind, World};
use crate::helpers::DocumentElementIcon;
use crate::helpers::DocumentElement;


mod render_types;
mod render_interface;
mod render_world;

pub fn render_interface(data: &DataProvider, interface: &Interface) -> Element {
    let words = data.package.worlds.iter().map(|(key, value)| value.left_link(data));
    let interfaces = data.package.interfaces.iter().map(|(key, value)| value.left_link(data));
    let card = interface.main_body(data);
    rsx! {
        div {
            class: "container",
            div { class: "lift-list", {words}, {interfaces} }
            div { class: "left-separator" }
            ul { class: "main-list" }
            {card}
        }
    }
}

fn make_link<T: DocumentElement + DocumentElementIcon>(item: &T, data: &DataProvider, class: &'static str) {
    match item.get_name(data) {
        "" => rsx! {},
        name => {
            let link = item.get_link(data);
            let icon = item.get_icon_name();
            rsx! {
               li {
                    class: class,
                    span { class: "type-icon", "{icon}" }
                    a { href: "{link}", "{name}" }
                }
            }

        },
    }
}
