use hunspell_rs::{CheckResult, Hunspell};
use std::path::Path;
use wit_parser::{Function, Interface, PackageName, TypeDef, UnresolvedPackage};

/// A spell checker for wit files.
pub struct WitSpellCheck {
    hunspell: Hunspell,
    check_function: bool,
    check_parameter: bool,
    check_type: bool,
}

impl WitSpellCheck {
    /// Create a new spell checker with the given hunspell directory.
    pub fn new<P>(directory: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = directory.as_ref();
        let aff = dir.join("en_US.aff");
        let dic = dir.join("en_US.dic");
        let mut hun = Hunspell::new(&aff.to_string_lossy(), &dic.to_string_lossy());
        // useless
        // hun.add_dictionary(&dir.join("custom.dic").to_string_lossy());
        let custom = std::fs::read_to_string(&dir.join("custom.dic"))?;
        for line in custom.lines() {
            if !line.trim().is_empty() {
                hun.add(line.trim());
            }
        }
        Ok(Self { hunspell: hun, check_function: true, check_parameter: true, check_type: true })
    }
    /// Whether to check function names.
    pub fn check_functions(self, check: bool) -> Self {
        Self { check_function: check, ..self }
    }
    /// Whether to check parameter names.
    pub fn check_parameters(self, check: bool) -> Self {
        Self { check_parameter: check, ..self }
    }
    /// Whether to check type names.
    pub fn check_type(self, check: bool) -> Self {
        Self { check_type: check, ..self }
    }
}

impl WitSpellCheck {
    /// Check the given directory for spelling errors.
    pub fn check<P>(&self, directory: P)
    where
        P: AsRef<Path>,
    {
        let package = UnresolvedPackage::parse_dir(directory.as_ref()).unwrap();
        for (_, interface) in package.interfaces.iter() {
            for (_, function) in interface.functions.iter() {
                if self.check_parameter {
                    self.check_parameter(function, interface, &package.name);
                }
                if self.check_function {
                    self.check_function(function, interface, &package.name);
                }
            }
            if self.check_type {
                for id in interface.types.values() {
                    match package.types.get(*id) {
                        Some(s) => match &s.name {
                            Some(s) => {
                                for part in s.split("-") {
                                    match self.hunspell.check(part) {
                                        CheckResult::FoundInDictionary => {}
                                        CheckResult::MissingInDictionary => {
                                            println!("- type may wrong: `{}`", part);
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
                            None => {}
                        },
                        None => {}
                    }
                }
            }
        }
    }
    /// Check the given function for spelling errors.
    fn check_function(&self, function: &Function, interface: &Interface, package: &PackageName) {
        let norm =
            function.name.trim_start_matches("[constructor]").trim_start_matches("[static]").trim_start_matches("[method]");
        for split in norm.split(".") {
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
    /// Check the given parameter for spelling errors.
    fn check_parameter(&self, function: &Function, interface: &Interface, package: &PackageName) {
        for (parameter, _) in function.params.iter() {
            for part in parameter.split("-") {
                match self.hunspell.check(part) {
                    CheckResult::FoundInDictionary => {}
                    CheckResult::MissingInDictionary => {
                        println!("- parameter may wrong: `{}`", part);
                        println!("  - in function `{}`", function.name);
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
