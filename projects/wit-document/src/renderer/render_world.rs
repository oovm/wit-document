
use super::*;

impl DocumentElement for World {
    fn get_name(&self, _: &DataProvider) -> &str {
        self.name.as_ref()
    }

    fn get_link(&self, data: &DataProvider) -> String {
        self.get_name(data).to_string()
    }


    fn left_link(&self, data: &DataProvider) -> Element {
        match self.get_name(data) {
            "" => rsx! {},
            name => {
                let link = self.get_link(data);


                rsx! {
               li {
                    class: "left-link",
                    span { class: "type-icon", "W" }
                    a { href: "{link}", "{name}" }
                }
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

    fn main_link(&self, data: &DataProvider) -> Element {
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

