use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
enum UnitVariants {
    Zero,
    One,
    Two,
}

#[test]
fn test_zero_unit() {
    let unit = UnitVariants::Zero;

    assert!(unit.as_zero().is_some());
    assert!(unit.as_one().is_none());
    assert!(unit.as_two().is_none());

    unit.as_zero().expect("expected ");
}

#[test]
fn test_one_unit() {
    let unit = UnitVariants::One;

    assert!(unit.as_zero().is_none());
    assert!(unit.as_one().is_some());
    assert!(unit.as_two().is_none());

    unit.as_one().expect("should have been some unit");
}

#[test]
fn test_two_unit() {
    let unit = UnitVariants::Two;

    assert!(unit.as_zero().is_none());
    assert!(unit.as_one().is_none());
    assert!(unit.as_two().is_some());

    unit.as_two().expect("should have been some unit");
}
