use super::*;
use crate::helpers::DocumentElement;
use std::borrow::Cow;

impl DocumentElementIcon for TypeDefKind {
    fn get_icon_name(&self) -> char {
        match self {
            Self::Record(_) => 'S',
            Self::Resource => 'R',
            Self::Handle(_) => 'T',
            Self::Flags(_) => 'F',
            Self::Tuple(_) => 'T',
            Self::Variant(_) => 'U',
            Self::Enum(_) => 'E',
            Self::Option(_) => 'T',
            Self::Result(_) => 'T',
            Self::List(_) => 'T',
            Self::Future(_) => 'T',
            Self::Stream(_) => 'T',
            Self::Type(_) => 'T',
            Self::Unknown => '?',
        }
    }
}

impl DocumentElementIcon for TypeDef {
    fn get_icon_name(&self) -> char {
        self.kind.get_icon_name()
    }
}

impl DocumentElement for TypeDef {
    fn get_name(&self, _: &DataProvider) -> Cow<str> {
        match self.name.as_ref() {
            Some(name) => Cow::Borrowed(name),
            None => Cow::Borrowed(""),
        }
    }

    fn get_link(&self, data: &DataProvider) -> String {
        self.get_name(data).to_string()
    }

    fn main_body(&self, data: &DataProvider) -> Element {
        todo!()
    }

    fn main_card(&self, data: &DataProvider) -> Element {
        match self.kind {
            TypeDefKind::Record(_) => panic!(),
            TypeDefKind::Resource => {
                let link = main_link(self, data);
                let document = match &self.docs.contents {
                    None => "",
                    Some(document) => document.lines().next().unwrap_or(""),
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
}
