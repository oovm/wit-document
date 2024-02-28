use super::*;


impl DocumentElement for Interface {
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
    fn main_body(&self, data: &DataProvider) -> Element {
        let resources = data.get_resources(&self.types).into_iter().map(|x| x.main_card(data));
        rsx! {
            div {
                class: "main-card",
                {resources}

            }
        }
    }

    fn main_card(&self, data: &DataProvider) -> Element {
        todo!()
    }
}
