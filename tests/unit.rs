#[macro_use]
extern crate enum_as_inner;

#[derive(EnumAsInner)]
enum UnitVariants {
    Zero = 0_isize,
    One,
    Two,
}

#[test]
fn test_zero_unit() {
    let unit = UnitVariants::Zero;

    assert!(unit.as_zero().is_some());
    assert!(unit.as_one().is_none());
    assert!(unit.as_two().is_none());

    assert_eq!(unit.as_zero().unwrap(), 0_isize);
}

#[test]
fn test_one_unit() {
    let unit = UnitVariants::One;

    assert!(unit.as_zero().is_none());
    assert!(unit.as_one().is_some());
    assert!(unit.as_two().is_none());

    assert_eq!(unit.as_one().unwrap(), 1_isize);
}

#[test]
fn test_two_unit() {
    let unit = UnitVariants::Two;

    assert!(unit.as_zero().is_none());
    assert!(unit.as_one().is_none());
    assert!(unit.as_two().is_some());

    assert_eq!(unit.as_two().unwrap(), 2_isize);
}