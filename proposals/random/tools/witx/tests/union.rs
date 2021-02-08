// Every worker needs a union! Time to organize
use witx::{Id, Representable};

#[test]
fn one_variant_union() {
    let d = witx::parse(
        "(typename $tag (enum $c))
         (typename $u (union (@witx tag $tag) u8))",
    );
    assert!(d.is_ok());
}

#[test]
fn two_variant_union() {
    let d1 = witx::parse(
        "(typename $tag (enum $a $b))
         (typename $u (union (@witx tag $tag) u8 u16))",
    );
    assert!(d1.is_ok(), "d1 is ok");

    // Fields can come in whatever order:
    let d2 = witx::parse(
        "(typename $tag (enum $a $b))
         (typename $u (variant (@witx tag $tag) (case $b u16) (case $a u8)))",
    );
    assert!(d2.is_ok(), "d2 is ok");

    // These two unions should be represented the same:
    let u1 = d1.unwrap().typename(&Id::new("u")).unwrap().type_();
    let u2 = d2.unwrap().typename(&Id::new("u")).unwrap().type_();

    assert_eq!(
        u1.representable(&u2),
        witx::RepEquality::Eq,
        "u1 can represent u2"
    );
    assert_eq!(
        u2.representable(&u1),
        witx::RepEquality::Eq,
        "u2 can represent u1"
    );

    // Tag order doesnt matter for validation
    let d3 = witx::parse(
        "(typename $tag (enum $b $a))
         (typename $u (union (@witx tag $tag) u16 u8))",
    );
    assert!(d3.is_ok(), "d2 is ok");
}

#[test]
fn empty_variant_unions() {
    let d1 = witx::parse(
        "(typename $tag (enum $a $b))
         (typename $u (variant (@witx tag $tag) (case $a) (case $b u16)))",
    );
    assert!(d1.is_ok(), "d1 is ok");

    let d2 = witx::parse(
        "(typename $tag (enum $a $b))
         (typename $u (variant (@witx tag $tag) (case $a) (case $b)))",
    );
    assert!(d2.is_ok(), "d2 is ok");
}

#[test]
fn many_variant_unions() {
    let d1 = witx::parse(
        "(typename $tag (enum $a $b $c $d $e $f $g $h $i $j $k $l $m))
         (typename $u
           (variant (@witx tag $tag)
            (case $a u8)
            (case $b u16)
            (case $c u32)
            (case $d u64)
            (case $e s8)
            (case $f s16)
            (case $g s32)
            (case $h s64)
            (case $i f32)
            (case $j f64)
            (case $k (@witx usize))
            (case $l char8)
            (case $m)
           )
         )",
    );
    assert!(d1.is_ok(), "d1 is ok");
}

#[test]
fn no_tag_union() {
    let d = witx::parse("(typename $u (union $tag (field $a u8) (field $b u16)))");
    assert!(d.is_err());
}

#[test]
fn wrong_kind_tag_union() {
    let d = witx::parse(
        "(typename $tag string)
            (typename $u (union (@witx tag $tag) u8 u16))",
    );
    let (expected, got) = wrong_kind_name_err(d).expect("wrong kind of tag");
    assert_eq!(expected, "enum or builtin");
    assert_eq!(got, "list");
}

#[test]
fn bad_field_unions() {
    let d = witx::parse(
        "(typename $tag (enum $c))
         (typename $u (variant (@witx tag $tag) (case $b u8)))",
    );
    match validation_err(d) {
        witx::ValidationError::InvalidUnionField { name, reason, .. } => {
            assert_eq!(name, "b", "bad field name union 1");
            assert_eq!(
                reason, "does not correspond to variant in tag `tag`",
                "reason union 1"
            );
        }
        other => panic!("bad error: {}", other),
    }

    let d = witx::parse(
        "(typename $tag (enum $c))
         (typename $u (variant (@witx tag $tag) (case $c f32) (case $b u8)))",
    );
    match validation_err(d) {
        witx::ValidationError::UnionSizeMismatch { .. } => {}
        other => panic!("bad error: {}", other),
    }

    let d = witx::parse(
        "(typename $tag (enum $c $d))
         (typename $u (variant (@witx tag $tag) (case $c f32)))",
    );
    match validation_err(d) {
        witx::ValidationError::UnionSizeMismatch { .. } => {}
        other => panic!("bad error: {}", other),
    }
}

fn wrong_kind_name_err(
    r: Result<witx::Document, witx::WitxError>,
) -> Option<(&'static str, &'static str)> {
    match r {
        Err(witx::WitxError::Validation(witx::ValidationError::WrongKindName {
            expected,
            got,
            ..
        })) => Some((expected, got)),
        Err(e) => {
            eprintln!("expected WrongKindName ValidationError, got: {:?}", e);
            None
        }
        Ok(_) => {
            eprintln!("expected WrongKindName ValidationError: Ok(witx::Document)");
            None
        }
    }
}

fn validation_err(r: Result<witx::Document, witx::WitxError>) -> witx::ValidationError {
    match r {
        Err(witx::WitxError::Validation(e)) => e,
        Err(e) => {
            panic!("expected ValidationError, got: {:?}", e)
        }
        Ok(_) => {
            panic!("expected ValidationError, got: Ok(witx::Document)")
        }
    }
}
