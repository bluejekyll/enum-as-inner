//! Unit tests for enum_as_inner crate.

use enum_as_inner::EnumAsInner;

pub mod name_collisions {
    #![allow(dead_code, missing_copy_implementations, missing_docs)]
    pub struct Option;
    pub struct Some;
    pub struct None;
    pub struct Result;
    pub struct Ok;
    pub struct Err;
}
#[allow(unused_imports)]
use name_collisions::*;

#[derive(EnumAsInner)]
enum UnitVariants {
    Zero,
    One,
    Two,
}

#[test]
fn test_zero_unit() {
    let unit = UnitVariants::Zero;

    assert!(unit.is_zero());
    assert!(!unit.is_one());
    assert!(!unit.is_two());
}

#[test]
fn test_one_unit() {
    let unit = UnitVariants::One;

    assert!(!unit.is_zero());
    assert!(unit.is_one());
    assert!(!unit.is_two());
}

#[test]
fn test_two_unit() {
    let unit = UnitVariants::Two;

    assert!(!unit.is_zero());
    assert!(!unit.is_one());
    assert!(unit.is_two());
}
