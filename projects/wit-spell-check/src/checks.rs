use std::borrow::Cow;
use std::collections::BTreeSet;
use wit_parser::UnresolvedPackage;

pub struct SpellCheck {
    package: UnresolvedPackage,
    reject: BTreeSet<Cow<'static, str>>,
    allow: BTreeSet<Cow<'static, str>>,
}