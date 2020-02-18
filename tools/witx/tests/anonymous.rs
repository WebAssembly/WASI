use witx;

fn is_anonymous_struct_err(r: Result<witx::Document, witx::WitxError>) -> bool {
    match r {
        Err(witx::WitxError::Validation(witx::ValidationError::AnonymousStructure { .. })) => true,
        _ => false,
    }
}

#[test]
fn anonymous_types() {
    let pointer_to_struct = witx::parse("(typename $a (@witx pointer (struct (field $b u8))))");
    assert!(is_anonymous_struct_err(pointer_to_struct));

    let pointer_to_union = witx::parse(
        "(typename $tag (enum u8 $b)) (typename $a (@witx pointer (union $tag (field $b u8))))",
    );
    assert!(is_anonymous_struct_err(pointer_to_union));

    let pointer_to_enum = witx::parse("(typename $a (@witx pointer (enum u32 $b)))");
    assert!(is_anonymous_struct_err(pointer_to_enum));

    let pointer_to_flags = witx::parse("(typename $a (@witx pointer (flags u32 $b)))");
    assert!(is_anonymous_struct_err(pointer_to_flags));

    let pointer_to_handle = witx::parse("(typename $a (@witx pointer (handle)))");
    assert!(is_anonymous_struct_err(pointer_to_handle));

    let pointer_to_builtin = witx::parse("(typename $a (@witx pointer u8))");
    assert!(pointer_to_builtin.is_ok());

    let pointer_to_pointer = witx::parse("(typename $a (@witx pointer (@witx const_pointer u8)))");
    assert!(pointer_to_pointer.is_ok());

    let struct_in_struct = witx::parse("(typename $a (struct (field $b (struct (field $c u8)))))");
    assert!(is_anonymous_struct_err(struct_in_struct));

    let union_in_struct = witx::parse(
        "(typename $tag (enum u8 $c)) (typename $a (struct (field $b (union $tag (field $c u8)))))",
    );
    assert!(is_anonymous_struct_err(union_in_struct));

    let pointer_in_struct = witx::parse("(typename $a (struct (field $b (@witx pointer u8))))");
    assert!(pointer_in_struct.is_ok())
}
