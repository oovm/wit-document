use crate::{
    helpers::{DocumentElement, DocumentElementIcon},
    DataProvider,
};
use dioxus::{
    html::{s, KeyCode::N},
    prelude::*,
};
use indexmap::IndexMap;
use wit_parser::{Function, Interface, TypeDef, TypeDefKind, TypeId, World};

mod render_functions;
mod render_interface;
mod render_types;
mod render_world;

pub fn render_interface(data: &DataProvider, interface: &Interface) -> Element {
    let words = data.package.worlds.iter().map(|(key, value)| make_link(value, data, "left-link"));
    let interfaces = data.package.interfaces.iter().map(|(key, value)| make_link(value, data, "left-link"));
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

fn make_link<T: DocumentElement + DocumentElementIcon>(item: &T, data: &DataProvider, class: &'static str) -> Element {
    match item.get_name(data).as_ref() {
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
        }
    }
}
fn main_link<T: DocumentElement + DocumentElementIcon>(item: &T, data: &DataProvider) -> Element {
    match item.get_name(data).as_ref() {
        "" => rsx! {},
        name => {
            let link = item.get_link(data);
            let icon = item.get_icon_name();
            rsx! {
               td {
                    class: "main-link",
                    span { class: "type-icon", "{icon}" }
                    a { href: "{link}", "{name}" }
                }
            }
        }
    }
}
fn main_resources<'a>(data: &'a DataProvider, item: &'a IndexMap<String, TypeId>) -> Element {
    let title = if data.has_resources(item) {
        rsx! {
             h2 {
                id: "resources",
                "Resources"
            }
        }
    }
    else {
        None
    };
    let terms = data.get_resources(item).into_iter().map(|x| x.main_card(data));
    rsx! {
        Fragment {
            {title}
            {terms}
        }
    }
}

fn main_functions<'a>(data: &'a DataProvider, item: &'a IndexMap<String, Function>) -> Element {
    let title = if data.has_functions(item) {
        rsx! {
             h2 {
                id: "functions",
                "Functions"
            }
        }
    }
    else {
        None
    };
    let terms = data.get_functions(item).into_iter().map(|x| x.main_card(data));
    rsx! {
        Fragment {
            {title}
            table {
                {terms}
            }
        }
    }
}
