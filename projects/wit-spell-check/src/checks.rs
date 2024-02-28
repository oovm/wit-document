use hunspell_rs::{CheckResult, Hunspell};
use std::path::Path;
use wit_parser::{Function, Interface, PackageName, TypeDefKind, UnresolvedPackage};

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
                self.check_types(interface, &package);
            }
        }
    }
    /// Check the given function for spelling errors.
    fn check_function(&self, function: &Function, interface: &Interface, package: &PackageName) {
        let norm =
            function.name.trim_start_matches("[constructor]").trim_start_matches("[static]").trim_start_matches("[method]");
        for split in norm.split(".") {
            self.check_kebab_case(split, |part| {
                println!("- function may wrong: `{}`", part);
                self.log_interface(interface);
                self.log_package(package);
                self.log_suggest(part);
            });
        }
    }
    /// Check the given parameter for spelling errors.
    fn check_parameter(&self, function: &Function, interface: &Interface, package: &PackageName) {
        for (parameter, _) in function.params.iter() {
            self.check_kebab_case(parameter, |part| {
                println!("- parameter may wrong: `{}`", part);
                self.log_interface(interface);
                self.log_package(package);
                self.log_suggest(part);
            });
        }
    }
    fn check_types(&self, interface: &Interface, package: &UnresolvedPackage) {
        for id in interface.types.values() {
            match package.types.get(*id) {
                Some(typing) => {
                    match &typing.name {
                        Some(s) => self.check_kebab_case(s, |part| {
                            println!("- type may wrong: `{}`", part);
                            self.log_interface(interface);
                            self.log_package(&package.name);
                            self.log_suggest(part);
                        }),
                        None => {}
                    }
                    match &typing.kind {
                        TypeDefKind::Record(_) => {}
                        TypeDefKind::Resource => {}
                        TypeDefKind::Handle(_) => {}
                        TypeDefKind::Flags(_) => {}
                        TypeDefKind::Tuple(_) => {}
                        TypeDefKind::Variant(_) => {}
                        TypeDefKind::Enum(_) => {}
                        TypeDefKind::Option(_) => {}
                        TypeDefKind::Result(_) => {}
                        TypeDefKind::List(_) => {}
                        TypeDefKind::Future(_) => {}
                        TypeDefKind::Stream(_) => {}
                        TypeDefKind::Type(_) => {}
                        TypeDefKind::Unknown => {}
                    }
                }
                None => {}
            }
        }
    }
    fn check_kebab_case(&self, word: &str, report: impl Fn(&str)) {
        for part in word.split("-") {
            match self.hunspell.check(word) {
                CheckResult::FoundInDictionary => {}
                CheckResult::MissingInDictionary => {
                    report(part);
                }
            }
        }
    }
    fn log_interface(&self, interface: &Interface) {
        if let Some(name) = interface.name.as_ref() {
            println!("  - in interface `{}`", name);
        }
    }
    fn log_package(&self, package: &PackageName) {
        println!("  - in package `{}`", package);
    }
    fn log_suggest(&self, word: &str) {
        println!("  - perhaps {:?}", self.hunspell.suggest(word));
    }
}
