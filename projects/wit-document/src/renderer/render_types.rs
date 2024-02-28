use dioxus::html::text;
use super::*;


impl DocumentElement for TypeDef {
    fn left_link(&self) -> Element {
        match self.kind {
            TypeDefKind::Record(_) => panic!(),
            TypeDefKind::Resource => panic!(),
            TypeDefKind::Handle(_) => panic!(),
            TypeDefKind::Flags(_) => panic!(),
            TypeDefKind::Tuple(_) => panic!(),
            TypeDefKind::Variant(_) => panic!(),
            TypeDefKind::Enum(_) => panic!(),
            TypeDefKind::Option(_) => panic!(),
            TypeDefKind::Result(_) => panic!(),
            TypeDefKind::List(_) => panic!(),
            TypeDefKind::Future(_) => panic!(),
            TypeDefKind::Stream(_) => panic!(),
            TypeDefKind::Type(_) => panic!(),
            TypeDefKind::Unknown => panic!(),
        }
    }

    fn main_body(&self, data: &DataProvider) -> Element {
        todo!()
    }

    fn main_card(&self, data: &DataProvider) -> Element {
        match self.kind {
            TypeDefKind::Record(_) => panic!(),
            TypeDefKind::Resource => {
                let link = self.main_link();
                let document = match &self.docs.contents {
                    None => { "" }
                    Some(document) => {
                        document.lines().next().unwrap_or("")
                    }
                };
                rsx! {
                    tr {
                        td {
                            class: "main-card-title",
                            {link}
                        }
                        td {
                            class: "main-card-detail",
                            {document}
                        }
                    }
                }
            }
            TypeDefKind::Handle(_) => panic!(),
            TypeDefKind::Flags(_) => panic!(),
            TypeDefKind::Tuple(_) => panic!(),
            TypeDefKind::Variant(_) => panic!(),
            TypeDefKind::Enum(_) => panic!(),
            TypeDefKind::Option(_) => panic!(),
            TypeDefKind::Result(_) => panic!(),
            TypeDefKind::List(_) => panic!(),
            TypeDefKind::Future(_) => panic!(),
            TypeDefKind::Stream(_) => panic!(),
            TypeDefKind::Type(_) => panic!(),
            TypeDefKind::Unknown => panic!(),
        }
    }

    fn main_link(&self) -> Element {
        match self.kind {
            TypeDefKind::Record(_) => panic!(),
            TypeDefKind::Resource => panic!(),
            TypeDefKind::Handle(_) => panic!(),
            TypeDefKind::Flags(_) => panic!(),
            TypeDefKind::Tuple(_) => panic!(),
            TypeDefKind::Variant(_) => panic!(),
            TypeDefKind::Enum(_) => panic!(),
            TypeDefKind::Option(_) => panic!(),
            TypeDefKind::Result(_) => panic!(),
            TypeDefKind::List(_) => panic!(),
            TypeDefKind::Future(_) => panic!(),
            TypeDefKind::Stream(_) => panic!(),
            TypeDefKind::Type(_) => panic!(),
            TypeDefKind::Unknown => panic!(),
        }
    }
}