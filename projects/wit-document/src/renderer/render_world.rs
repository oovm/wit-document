use super::*;

impl DocumentElement for World {
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

    fn main_body(&self, data: &DataProvider) -> Element {
        todo!()
    }

    fn main_card(&self, data: &DataProvider) -> Element {
        todo!()
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
