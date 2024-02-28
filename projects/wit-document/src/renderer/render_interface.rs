
use super::*;


impl DocumentElementIcon for Interface {
    fn get_icon_name(&self) -> char {
        'I'
    }
}

impl DocumentElement for Interface {
    fn get_name(&self, _: &DataProvider) -> &str {
        match self.name.as_ref() {
            Some(name) => name,
            None => ""
        }
    }

    fn get_link(&self, data: &DataProvider) -> String {
        let interface = self.name.as_ref().expect("Check for empty interface name first!");
        let package = &data.package.name;
        match package.version.as_ref() {
            Some(version) =>
                format!(
                    "/{}:{}/{}@{}",
                    package.namespace,
                    package.name,
                    interface,
                    version,
                ),
            None =>
                format!(
                    "/{}:{}/{}",
                    package.namespace,
                    package.name,
                    interface,
                )
        }
    }

    fn left_link(&self, data: &DataProvider) -> Element {
        match self.get_name(data) {
            "" => rsx! {},
            name => {
                let link = self.get_link(data);
                rsx! {
                   li {
                        class: "left-link",
                        span { class: "type-icon", "I" }
                        a { href: link, "{name}" }
                    }
                }
            }
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

    fn main_link(&self, data: &DataProvider) -> Element {
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
