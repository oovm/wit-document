use crate::DataProvider;
use dioxus::{html::s, prelude::*};
use wit_parser::{Interface, World};

pub fn render_package(data: &DataProvider) -> Element {
    let words = data.package.worlds.iter().map(|(key, value)| value.left_link());
    let interfaces = data.package.interfaces.iter().map(|(key, value)| value.left_link());
    rsx! {
        div {
            class: "container",
            div {
                class: "lift-list",
                {words}
                {interfaces}
            }
            div {
                class: "left-separator"
            }
            ul {
                class: "main-list",
            }
        }
    }
}

pub trait LeftLink {
    fn left_link(&self) -> Element;
    fn main_link(&self) -> Element;
}

impl<'a> LeftLink for &'a World {
    fn left_link(&self) -> Element {
        match self.name.as_str() {
            "" => rsx! {},
            value => rsx! {
               li {
                    class: "left-link",
                    span { class: "type-icon", "W" }
                    a { href: "{value}", "{value}" }
                }
            },
        }
    }

    fn main_link(&self) -> Element {
        match self.name.as_str() {
            "" => rsx! {},
            value => rsx! {
               li {
                    class: "main-link",
                    span { class: "type-icon", "W" }
                    a { href: "{value}", "{value}" }
                }
            },
        }
    }
}

impl<'a> LeftLink for &'a Interface {
    fn left_link(&self) -> Element {
        match self.name.as_ref() {
            Some(value) => {
                rsx! {
                   li {
                        class: "left-link",
                        span { class: "type-icon", "I" }
                        a { href: "{value}", "{value}" }
                    }
                }
            }
            None => rsx! {},
        }
    }

    fn main_link(&self) -> Element {
        match self.name.as_ref() {
            Some(value) => {
                rsx! {
                   li {
                        class: "main-link",
                        span { class: "type-icon", "I" }
                        a { href: "{value}", "{value}" }
                    }
                }
            }
            None => rsx! {},
        }
    }
}
