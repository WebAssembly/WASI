// Every worker needs a union! Time to organize
use witx::{Id, Representable};

#[test]
fn one_variant_union() {
    let d = witx::parse(
        "(typename $tag (enum u8 $c))
         (typename $u (union $tag (field $c u8)))",
    );
    assert!(d.is_ok());
}

#[test]
fn two_variant_union() {
    let d1 = witx::parse(
        "(typename $tag (enum u8 $a $b))
         (typename $u (union $tag (field $a u8) (field $b u16)))",
    );
    assert!(d1.is_ok(), "d1 is ok");

    // Fields can come in whatever order:
    let d2 = witx::parse(
        "(typename $tag (enum u8 $a $b))
         (typename $u (union $tag (field $b u16) (field $a u8)))",
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

    // Tag order doesnt matter for validation, but does for rep equality
    let d3 = witx::parse(
        "(typename $tag (enum u8 $b $a))
         (typename $u (union $tag (field $b u16) (field $a u8)))",
    );
    assert!(d3.is_ok(), "d2 is ok");
    let u3 = d3.unwrap().typename(&Id::new("u")).unwrap().type_();
    assert_eq!(
        u3.representable(&u1),
        witx::RepEquality::NotEq,
        "u3 cannot represent u1"
    );
}

#[test]
fn empty_variant_unions() {
    let d1 = witx::parse(
        "(typename $tag (enum u8 $a $b))
         (typename $u (union $tag (empty $a) (field $b u16)))",
    );
    assert!(d1.is_ok(), "d1 is ok");

    let d2 = witx::parse(
        "(typename $tag (enum u8 $a $b))
         (typename $u (union $tag (empty $a) (empty $b)))",
    );
    assert!(d2.is_ok(), "d2 is ok");
}

#[test]
fn many_variant_unions() {
    let d1 = witx::parse(
        "(typename $tag (enum u32 $a $b $c $d $e $f $g $h $i $j $k $l $m))
         (typename $u
           (union $tag
            (field $a u8)
            (field $b u16)
            (field $c u32)
            (field $d u64)
            (field $e s8)
            (field $f s16)
            (field $g s32)
            (field $h s64)
            (field $i f32)
            (field $j f64)
            (field $k (@witx usize))
            (field $l char8)
            (empty $m)
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
        "(typename $tag u32)
            (typename $u (union $tag (field $a u8) (field $b u16)))",
    );
    let (expected, got) = wrong_kind_name_err(d).expect("wrong kind of tag");
    assert_eq!(expected, "enum");
    assert_eq!(got, "builtin");
}

#[test]
fn bad_field_unions() {
    let d = witx::parse(
        "(typename $tag (enum u8 $c))
         (typename $u (union $tag (field $b u8)))",
    );
    let (name, reason) = union_field_err(d).expect("bad field union 1");
    assert_eq!(name, "b", "bad field name union 1");
    assert_eq!(
        reason, "does not correspond to variant in tag `tag`",
        "reason union 1"
    );

    let d = witx::parse(
        "(typename $tag (enum u8 $c))
         (typename $u (union $tag (field $c f32) (field $b u8)))",
    );
    let (name, reason) = union_field_err(d).expect("bad field union 2");
    assert_eq!(name, "b", "bad field name union 2");
    assert_eq!(
        reason, "does not correspond to variant in tag `tag`",
        "reason union 2"
    );

    let d = witx::parse(
        "(typename $tag (enum u8 $c $d))
         (typename $u (union $tag (field $c f32)))",
    );
    let (name, reason) = union_field_err(d).expect("bad field union 3");
    assert_eq!(name, "d", "bad field name union 3");
    assert_eq!(reason, "missing variants from tag `tag`", "reason union 3");
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

fn union_field_err(r: Result<witx::Document, witx::WitxError>) -> Option<(String, String)> {
    match r {
        Err(witx::WitxError::Validation(witx::ValidationError::InvalidUnionField {
            name,
            reason,
            ..
        })) => Some((name, reason)),
        Err(e) => {
            eprintln!("expected InvalidUnionField ValidationError, got: {:?}", e);
            None
        }
        Ok(_) => {
            eprintln!("expected InvalidUnionField ValidationError, got: Ok(witx::Document)");
            None
        }
    }
}
