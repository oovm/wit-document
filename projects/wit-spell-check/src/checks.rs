use std::borrow::Cow;
use std::collections::BTreeSet;
use std::path::Path;
use hunspell_rs::Hunspell;
use wit_parser::{TypeDef, UnresolvedPackage};

pub struct WitSpellCheck {
    hunspell: Hunspell,
    accept: BTreeSet<Cow<'static, str>>,
    reject: BTreeSet<Cow<'static, str>>,
}

impl Default for WitSpellCheck {
    fn default() -> Self {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/en_US");
        Self {
            hunspell: Hunspell::new(
                path.join("en_US.aff").to_str().unwrap(),
                path.join("en_US.dic").to_str().unwrap(),
            ),
            accept: Default::default(),
            reject: Default::default(),
        }
    }
}

impl WitSpellCheck {
    pub fn check<P: AsRef<Path>>(&self, directory: P) {
        let package = UnresolvedPackage::parse_dir(directory.as_ref()).unwrap();
        for (_, interface) in package.interfaces.iter() {
            for (name, function) in interface.functions.iter() {
                for (parameter, _) in function.params.iter() {
                    if !self.hunspell.check(parameter) {
                        println!("{}: {}", name, parameter);
                    }
                }
            }
        }
    }
}