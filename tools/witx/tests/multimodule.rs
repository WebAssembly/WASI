use witx::{load, BuiltinType, Id, Type, TypeRef};

#[test]
fn validate_multimodule() {
    // B uses A, and C uses A.
    let doc = load(&[
        "tests/multimodule/type_b.witx",
        "tests/multimodule/type_c.witx",
    ])
    .unwrap_or_else(|e| panic!("failed to validate: {}", e));

    //println!("{}", doc);

    // Check that the `a` both modules use is what we expect:
    let type_a = doc.typename(&Id::new("a")).expect("type a exists");
    assert_eq!(*type_a.type_(), Type::Builtin(BuiltinType::U32));

    // `b` is a struct with a single member of type `a`
    let type_b = doc.typename(&Id::new("b")).expect("type b exists");
    match &*type_b.type_() {
        Type::Struct(struct_) => {
            assert_eq!(struct_.members.len(), 1);
            assert_eq!(
                struct_.members.get(0).unwrap().tref,
                TypeRef::Name(type_a.clone())
            );
        }
        _ => panic!("b is a struct"),
    }

    // `c` is a struct with a two members of type `a`
    let type_c = doc.typename(&Id::new("c")).expect("type c exists");
    match &*type_c.type_() {
        Type::Struct(struct_) => {
            assert_eq!(struct_.members.len(), 2);
            assert_eq!(
                struct_.members.get(0).unwrap().tref,
                TypeRef::Name(type_a.clone())
            );
            assert_eq!(struct_.members.get(1).unwrap().tref, TypeRef::Name(type_a));
        }
        _ => panic!("c is a struct"),
    }
}

#[test]
fn multimodule_reject_redefinition() {
    assert!(load(&[
        "tests/multimodule/type_a.witx",
        "tests/multimodule/redefine_a.witx",
    ])
    .is_err())
}
