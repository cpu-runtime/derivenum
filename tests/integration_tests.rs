#![cfg(test)]
#![allow(dead_code)]

use derivenum::{EnumMatch, EnumTake};

#[derive(EnumMatch)]
enum EnumFields {
    Unnamed(String),
    Named { name1: u32, name2: String },
    Unit,
}

#[test]
pub fn unnamed_fields() {
    let unnamed = EnumFields::Unnamed(String::new());
    assert!(unnamed.am_unnamed());
    assert!(!unnamed.am_named());
    assert!(!unnamed.am_unit());
}
#[test]
pub fn named_fields() {
    let named = EnumFields::Named {
        name1: 1,
        name2: String::from("2"),
    };
    assert!(named.am_named());
    assert!(!named.am_unnamed());
    assert!(!named.am_unit());
}
#[test]
pub fn unit_fields() {
    let unit = EnumFields::Unit;
    assert!(unit.am_unit());
    assert!(!unit.am_named());
    assert!(!unit.am_unnamed());
}

#[derive(EnumTake)]
enum Takeable {
    SingleUnnamed(String),
    MultiUnnamed(u32, u32, u32),
    SingleNamed { name: String },
    MultiNamed { field: u32, field2: i32 },
    Unit,
}

#[test]
pub fn take_values() {
    let takeable = Takeable::SingleUnnamed(String::new());
    assert_eq!(takeable.take_single_unnamed(), String::new());
    let takeable = Takeable::MultiUnnamed(1, 2, 3);
    assert_eq!(takeable.take_multi_unnamed(), (1, 2, 3));
}
