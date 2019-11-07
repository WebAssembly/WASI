use witx::{load, BuiltinType, DatatypeIdent, DatatypeVariant, Id};

#[test]
fn validate_multimodule() {
    // B uses A, and C uses A.
    let doc = load(&[
        "tests/multimodule/type_b.witx",
        "tests/multimodule/type_c.witx",
    ])
    .unwrap_or_else(|e| panic!("failed to validate: {}", e));

    println!("{}", doc);

    // Check that the `a` both modules use is what we expect:
    let type_a = doc.datatype(&Id::new("a")).expect("type a exists");
    match &type_a.variant {
        DatatypeVariant::Alias(alias) => match alias.to {
            DatatypeIdent::Builtin(b) => assert_eq!(b, BuiltinType::U32),
            _ => panic!("a is an alias u32"),
        },
        _ => panic!("a is an alias to u32"),
    }

    // `b` is a struct with a single member of type `a`
    let type_b = doc.datatype(&Id::new("b")).expect("type b exists");
    match &type_b.variant {
        DatatypeVariant::Struct(struct_) => {
            assert_eq!(struct_.members.len(), 1);
            match &struct_.members.get(0).unwrap().type_ {
                DatatypeIdent::Ident(member_a) => assert_eq!(*member_a, type_a),
                _ => panic!("b.0 has type a"),
            }
        }
        _ => panic!("b is a struct"),
    }

    // `c` is a struct with a two members of type `a`
    let type_c = doc.datatype(&Id::new("c")).expect("type c exists");
    match &type_c.variant {
        DatatypeVariant::Struct(struct_) => {
            assert_eq!(struct_.members.len(), 2);
            match &struct_.members.get(0).unwrap().type_ {
                DatatypeIdent::Ident(member_a) => assert_eq!(*member_a, type_a),
                _ => panic!("c.0 has type a"),
            }
            match &struct_.members.get(1).unwrap().type_ {
                DatatypeIdent::Ident(member_a) => assert_eq!(*member_a, type_a),
                _ => panic!("c.1 has type a"),
            }
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
