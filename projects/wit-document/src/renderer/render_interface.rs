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

    fn main_body(&self, data: &DataProvider) -> Element {
        let resources = main_resources(data, &self.types);
        let functions = main_functions(data, &self.functions);
        rsx! {
            div {
                class: "main-card",
                {resources}
                {functions}
            }
        }
    }

    fn main_card(&self, data: &DataProvider) -> Element {
        todo!()
    }
}
