use std::path::Path;
use wit_spell_check::WitSpellCheck;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() {
    let spell = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/en_US");
    let checker = WitSpellCheck::new(spell);
    let mut wits = Path::new(env!("CARGO_MANIFEST_DIR")).join("../wit-document/tests/preview2");
    // find all directories
    for entry in wits.read_dir().unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            checker.check(entry.path());
        }
    }
}