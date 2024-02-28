use super::*;


impl DocumentElement for TypeDef {
    fn left_link(&self) -> Element {
        match self.kind {
            TypeDefKind::Record(_) => rsx! { " TypeDefKind::Record"}
            TypeDefKind::Resource => rsx! { " TypeDefKind::Resource"}
            TypeDefKind::Handle(_) => rsx! { " TypeDefKind::Handle"}
            TypeDefKind::Flags(_) => rsx! { " TypeDefKind::Flags"}
            TypeDefKind::Tuple(_) => rsx! { " TypeDefKind::Tuple"}
            TypeDefKind::Variant(_) => rsx! { " TypeDefKind::Variant"}
            TypeDefKind::Enum(_) => rsx! { " TypeDefKind::Enum"}
            TypeDefKind::Option(_) => rsx! {    " TypeDefKind::Option"}
            TypeDefKind::Result(_) => rsx! {    " TypeDefKind::Result"}
            TypeDefKind::List(_) => rsx! {   " TypeDefKind::List"}
            TypeDefKind::Future(_) => rsx! { " TypeDefKind::Future"}
            TypeDefKind::Stream(_) => rsx! { " TypeDefKind::Stream"}
            TypeDefKind::Type(_) => rsx! {  " TypeDefKind::Type"}
            TypeDefKind::Unknown => rsx! { " TypeDefKind::Unknown"}
        }
    }

    fn main_card(&self, data: &DataProvider) -> Element {
        todo!()
    }

    fn main_link(&self) -> Element {
        match self.kind {
            TypeDefKind::Record(_) => rsx! { " TypeDefKind::Record"}
            TypeDefKind::Resource => rsx! { " TypeDefKind::Resource"}
            TypeDefKind::Handle(_) => rsx! { " TypeDefKind::Handle"}
            TypeDefKind::Flags(_) => rsx! { " TypeDefKind::Flags"}
            TypeDefKind::Tuple(_) => rsx! { " TypeDefKind::Tuple"}
            TypeDefKind::Variant(_) => rsx! { " TypeDefKind::Variant"}
            TypeDefKind::Enum(_) => rsx! { " TypeDefKind::Enum"}
            TypeDefKind::Option(_) => rsx! {    " TypeDefKind::Option"}
            TypeDefKind::Result(_) => rsx! {    " TypeDefKind::Result"}
            TypeDefKind::List(_) => rsx! {   " TypeDefKind::List"}
            TypeDefKind::Future(_) => rsx! { " TypeDefKind::Future"}
            TypeDefKind::Stream(_) => rsx! { " TypeDefKind::Stream"}
            TypeDefKind::Type(_) => rsx! {  " TypeDefKind::Type"}
            TypeDefKind::Unknown => rsx! { " TypeDefKind::Unknown"}
        }
    }
}