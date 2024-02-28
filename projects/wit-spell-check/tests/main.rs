use std::path::Path;
use wit_spell_check::WitSpellCheck;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() {
    let checker = WitSpellCheck::default();
    let mut here = Path::new(env!("CARGO_MANIFEST_DIR")).join("../wit-document/tests/preview2");
    // find all directories
    for entry in here.read_dir().unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            checker.check(entry.path());
        }
    }

}