use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
enum EmptyTest {}

#[test]
fn test_empty() {
    let empty = None::<EmptyTest>;

    assert!(empty.is_none());
}

#[derive(EnumAsInner)]
enum EmptyParendsTest {
    Empty(),
}

#[test]
fn test_empty_parends() {
    let empty = EmptyParendsTest::Empty();

    assert_eq!(empty.as_empty().unwrap(), ());
}

#[derive(EnumAsInner)]
enum OneTest {
    One(u32),
}

#[test]
fn test_one() {
    let empty = OneTest::One(1);

    assert_eq!(*empty.as_one().unwrap(), 1);
}

#[derive(EnumAsInner)]
enum MultiTest {
    Multi(u32, u32),
}

#[test]
fn test_multi() {
    let multi = MultiTest::Multi(1, 1);

    assert_eq!(multi.as_multi().unwrap(), (&1_u32, &1_u32));
}
