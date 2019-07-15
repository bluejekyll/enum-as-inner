use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
enum MixedCaseVariants {
    XMLIsNotCool,
    #[allow(non_camel_case_types)]
    Rust_IsCoolThough(u32),
    YMCA {
        named: i16,
    },
}

#[test]
fn test_xml_unit() {
    let mixed = MixedCaseVariants::XMLIsNotCool;

    assert!(mixed.as_xml_is_not_cool().is_some());
    assert!(mixed.as_rust_is_cool_though().is_none());
    assert!(mixed.as_ymca().is_none());

    assert_eq!(mixed.as_xml_is_not_cool().unwrap(), ());
}

#[test]
fn test_rust_unnamed() {
    let mixed = MixedCaseVariants::Rust_IsCoolThough(42);

    assert!(mixed.as_xml_is_not_cool().is_none());
    assert!(mixed.as_rust_is_cool_though().is_some());
    assert!(mixed.as_ymca().is_none());

    assert_eq!(*mixed.as_rust_is_cool_though().unwrap(), 42);
}

#[test]
fn test_ymca_named() {
    let mixed = MixedCaseVariants::YMCA { named: -32_768 };

    assert!(mixed.as_xml_is_not_cool().is_none());
    assert!(mixed.as_rust_is_cool_though().is_none());
    assert!(mixed.as_ymca().is_some());

    assert_eq!(*mixed.as_ymca().unwrap(), (-32_768));
}
