use std::borrow::Cow;
use std::collections::BTreeSet;
use std::path::Path;
use hunspell_rs::{CheckResult, Hunspell};
use wit_parser::{TypeDef, UnresolvedPackage};

pub struct WitSpellCheck {
    hunspell: Hunspell,
    check_function: bool,
    check_parameter: bool,
}

impl WitSpellCheck {
    pub fn new<P>(directory: P) -> Self where P: AsRef<Path> {
        let dir = directory.as_ref();
        let aff = dir.join("en_US.aff");
        let dic = dir.join("en_US.dic");
        let mut hun = Hunspell::new(&aff.to_string_lossy(), &dic.to_string_lossy());
        hun.add_dictionary(&dir.join("custom.dic").to_string_lossy());
        Self {
            hunspell: hun,
            check_function: true,
            check_parameter: true,
        }
    }
    pub fn check_functions(self, check: bool) -> Self {
        Self {
            check_function: check,
            ..self
        }
    }
    pub fn check_parameters(self, check: bool) -> Self {
        Self {
            check_parameter: check,
            ..self
        }
    }
}

impl WitSpellCheck {
    pub fn check<P>(&self, directory: P) where P: AsRef<Path> {
        let package = UnresolvedPackage::parse_dir(directory.as_ref()).unwrap();
        for (_, interface) in package.interfaces.iter() {
            for (name, function) in interface.functions.iter() {
                for (parameter, _) in function.params.iter() {
                    for part in parameter.split("-") {
                        match self.hunspell.check(part) {
                            CheckResult::FoundInDictionary => {}
                            CheckResult::MissingInDictionary => {
                                println!("- parameter may wrong: `{}`", part);
                                println!("  - in function `{}`", name);
                                match &interface.name {
                                    Some(s) => {
                                        println!("  - in interface `{}`", s);
                                    }
                                    None => {}
                                }
                                println!("  - in package `{}`", package.name);
                                println!("  - perhaps {:?}", self.hunspell.suggest(part));
                            }
                        }
                    }
                }
                let trim = name.trim_start_matches("[constructor]").trim_start_matches("[static]").trim_start_matches("[method]");
                for split in trim.split(".") {
                    for part in split.split("-") {
                        match self.hunspell.check(part) {
                            CheckResult::FoundInDictionary => {}
                            CheckResult::MissingInDictionary => {
                                println!("- function may wrong: `{}`", part);
                                match &interface.name {
                                    Some(s) => {
                                        println!("  - in interface `{}`", s);
                                    }
                                    None => {}
                                }
                                println!("  - in package `{}`", package.name);
                                println!("  - perhaps {:?}", self.hunspell.suggest(part));
                            }
                        }
                    }
                }
            }
        }
    }
}