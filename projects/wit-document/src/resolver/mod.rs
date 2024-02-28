use dioxus::{core_macro::rsx, dioxus_core::Element};
use std::{path::Path, process::id};
use wit_parser::{
    Enum, EnumCase, Flags, Function, FunctionKind, Interface, Resolve, TypeDef, TypeDefKind, UnresolvedPackage, Variant,
};

#[test]
fn test() -> anyhow::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut store = DataProvider { package: UnresolvedPackage::parse_dir(&here.join("tests/preview2/cli"))? };
    for (_, interface) in store.package.interfaces.iter() {
        println!("=== {:?} ===", interface.name);
        for resource in store.get_resources(interface) {
            println!("{:?}", resource);
        }
        for (ty, flags) in store.get_flags(interface) {
            println!("{:?}", ty);
        }
        for (ty, enumerate) in store.get_enumerate(interface) {
            println!("{:?}", ty);
        }
        for (ty, variant) in store.get_variant(interface) {
            println!("{:?}", ty);
        }

        for resource in store.get_functions(interface) {
            println!("{:?}", resource);
        }
    }
    Ok(())
}

pub struct DataProvider {
    pub package: UnresolvedPackage,
}

impl DataProvider {
    pub fn get_resources<'a>(&'a self, interface: &'a Interface) -> Vec<&'a TypeDef> {
        let mut resources = vec![];
        for ty in interface.types.values() {
            match self.package.types.get(*ty) {
                Some(s) => match s.kind {
                    TypeDefKind::Resource => {
                        resources.push(s);
                    }
                    _ => {}
                },
                None => {}
            }
        }
        resources
    }
    pub fn get_flags<'a>(&'a self, interface: &'a Interface) -> Vec<(&'a TypeDef, &'a Flags)> {
        let mut resources = vec![];
        for ty in interface.types.values() {
            match self.package.types.get(*ty) {
                Some(s) => match &s.kind {
                    TypeDefKind::Flags(flags) => {
                        resources.push((s, flags));
                    }
                    _ => {}
                },
                None => {}
            }
        }
        resources
    }
    pub fn get_enumerate<'a>(&'a self, interface: &'a Interface) -> Vec<(&'a TypeDef, &'a Enum)> {
        let mut resources = vec![];
        for ty in interface.types.values() {
            match self.package.types.get(*ty) {
                Some(s) => match &s.kind {
                    TypeDefKind::Enum(e) => {
                        resources.push((s, e));
                    }
                    _ => {}
                },
                None => {}
            }
        }
        resources
    }
    pub fn get_variant<'a>(&'a self, interface: &'a Interface) -> Vec<(&'a TypeDef, &'a Variant)> {
        let mut resources = vec![];
        for ty in interface.types.values() {
            match self.package.types.get(*ty) {
                Some(s) => match &s.kind {
                    TypeDefKind::Variant(v) => {
                        resources.push((s, v));
                    }
                    _ => {}
                },
                None => {}
            }
        }
        resources
    }

    pub fn get_functions<'a>(&'a self, interface: &'a Interface) -> Vec<&'a Function> {
        interface.functions.values().filter(|x| x.kind == FunctionKind::Freestanding).collect()
    }
}
