use witx;

fn is_anonymous_record_err(r: Result<witx::Document, witx::WitxError>) -> bool {
    match r {
        Err(witx::WitxError::Validation(witx::ValidationError::AnonymousRecord { .. })) => true,
        _ => false,
    }
}

#[test]
fn anonymous_types() {
    let pointer_to_record = witx::parse("(typename $a (@witx pointer (record (field $b u8))))");
    assert!(is_anonymous_record_err(pointer_to_record));

    let pointer_to_union = witx::parse(
        "(typename $tag (enum $b)) (typename $a (@witx pointer (union $tag (field $b u8))))",
    );
    assert!(is_anonymous_record_err(pointer_to_union));

    let pointer_to_enum = witx::parse("(typename $a (@witx pointer (enum $b)))");
    assert!(is_anonymous_record_err(pointer_to_enum));

    let pointer_to_flags = witx::parse("(typename $a (@witx pointer (flags u32 $b)))");
    assert!(is_anonymous_record_err(pointer_to_flags));

    let pointer_to_handle = witx::parse("(typename $a (@witx pointer (handle)))");
    assert!(is_anonymous_record_err(pointer_to_handle));

    let pointer_to_builtin = witx::parse("(typename $a (@witx pointer u8))");
    assert!(pointer_to_builtin.is_ok());

    let pointer_to_pointer = witx::parse("(typename $a (@witx pointer (@witx const_pointer u8)))");
    assert!(pointer_to_pointer.is_ok());

    let record_in_record = witx::parse("(typename $a (record (field $b (record (field $c u8)))))");
    assert!(is_anonymous_record_err(record_in_record));

    let union_in_record = witx::parse(
        "(typename $tag (enum $c)) (typename $a (record (field $b (union $tag (field $c u8)))))",
    );
    assert!(is_anonymous_record_err(union_in_record));

    let pointer_in_record = witx::parse("(typename $a (record (field $b (@witx pointer u8))))");
    assert!(pointer_in_record.is_ok())
}
