use crate::{BuiltinType, IntRepr, NamedType, RecordDatatype, Type, TypeRef, Variant};
use std::collections::HashMap;

// A lattice. Eq + Eq = Eq, SuperSet + any = NotEq, NotEq + any = NotEq.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RepEquality {
    Eq,
    Superset,
    NotEq,
}

impl RepEquality {
    pub fn join(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (RepEquality::Eq, RepEquality::Eq) => RepEquality::Eq,
            _ => RepEquality::NotEq,
        }
    }
}

pub trait Representable {
    fn representable(&self, by: &Self) -> RepEquality;
}

impl Representable for BuiltinType {
    fn representable(&self, by: &Self) -> RepEquality {
        // An unsigned integer can be used to represent an unsigned integer of smaller width.
        // Otherwise, types must be equal.
        if self == by {
            return RepEquality::Eq;
        }
        match self {
            BuiltinType::U8 => match by {
                BuiltinType::U64 | BuiltinType::U32 | BuiltinType::U16 => RepEquality::Superset,
                _ => RepEquality::NotEq,
            },
            BuiltinType::U16 => match by {
                BuiltinType::U64 | BuiltinType::U32 => RepEquality::Superset,
                _ => RepEquality::NotEq,
            },
            BuiltinType::U32 => match by {
                BuiltinType::U64 => RepEquality::Superset,
                _ => RepEquality::NotEq,
            },
            _ => RepEquality::NotEq,
        }
    }
}

impl Representable for IntRepr {
    fn representable(&self, by: &Self) -> RepEquality {
        if self == by {
            return RepEquality::Eq;
        }
        // An unsigned integer can be used to represent an unsigned integer of smaller width.
        match self {
            IntRepr::U16 => match by {
                IntRepr::U32 | IntRepr::U64 => RepEquality::Superset,
                _ => RepEquality::NotEq,
            },
            IntRepr::U32 => match by {
                IntRepr::U64 => RepEquality::Superset,
                _ => RepEquality::NotEq,
            },
            _ => RepEquality::NotEq,
        }
    }
}

impl Representable for Variant {
    fn representable(&self, by: &Self) -> RepEquality {
        let mut superset = false;
        // Integer representation must be compatible
        match self.tag_repr.representable(&by.tag_repr) {
            RepEquality::NotEq => return RepEquality::NotEq,
            RepEquality::Eq => {}
            RepEquality::Superset => superset = true,
        }
        let other_by_name = by
            .cases
            .iter()
            .map(|c| (&c.name, c))
            .collect::<HashMap<_, _>>();
        // For each variant in self, must have variant of same name in by:
        for v in self.cases.iter() {
            let other_ty = match other_by_name.get(&v.name) {
                Some(other) => &other.tref,
                None => return RepEquality::NotEq,
            };
            match (&v.tref, other_ty) {
                (Some(me), Some(other)) => match me.representable(other) {
                    RepEquality::NotEq => return RepEquality::NotEq,
                    RepEquality::Eq => {}
                    RepEquality::Superset => superset = true,
                },
                // We added fields, that's not ok
                (Some(_), None) => return RepEquality::NotEq,
                // Fields were deleted, that's ok
                (None, Some(_)) => superset = true,
                (None, None) => {}
            }
        }
        if superset || self.cases.len() < by.cases.len() {
            RepEquality::Superset
        } else {
            RepEquality::Eq
        }
    }
}

impl Representable for RecordDatatype {
    fn representable(&self, by: &Self) -> RepEquality {
        // Records must have exact structural equality - same members, must
        // be Eq, in the same order.
        // We would require require a more expressive RepEquality enum to describe which members
        // might be supersets.
        if self.members.len() != by.members.len() {
            return RepEquality::NotEq;
        }
        for (m, bym) in self.members.iter().zip(by.members.iter()) {
            if m.name != bym.name {
                return RepEquality::NotEq;
            }
            if m.tref.type_().representable(&*bym.tref.type_()) != RepEquality::Eq {
                return RepEquality::NotEq;
            }
        }
        RepEquality::Eq
    }
}

impl Representable for TypeRef {
    fn representable(&self, by: &Self) -> RepEquality {
        self.type_().representable(&*by.type_())
    }
}

impl Representable for NamedType {
    fn representable(&self, by: &Self) -> RepEquality {
        self.tref.representable(&by.tref)
    }
}

impl Representable for Type {
    fn representable(&self, by: &Self) -> RepEquality {
        match (&self, &by) {
            (Type::Variant(s), Type::Variant(b)) => s.representable(b),
            (Type::Record(s), Type::Record(b)) => s.representable(b),
            (Type::Handle(_), Type::Handle(_)) => RepEquality::Eq, // Handles are nominal, not structural
            (Type::List(s), Type::List(b)) => s.representable(b),
            (Type::Pointer(s), Type::Pointer(b)) => s.representable(b),
            (Type::ConstPointer(s), Type::ConstPointer(b)) => s.representable(b),
            (Type::Builtin(s), Type::Builtin(b)) => s.representable(b),
            _ => RepEquality::NotEq,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::io::MockFs;
    use crate::toplevel::parse_witx_with;
    use crate::Id;
    use std::rc::Rc;

    fn def_type(typename: &str, syntax: &str) -> Rc<NamedType> {
        use std::path::Path;
        let doc = parse_witx_with(&[Path::new("-")], &MockFs::new(&[("-", syntax)]))
            .expect("parse witx doc");
        let t = doc.typename(&Id::new(typename)).expect("defined type");
        // Identity should always be true:
        assert_eq!(t.representable(&t), RepEquality::Eq, "identity");
        t
    }

    #[test]
    fn different_typenames() {
        let a = def_type("a", "(typename $a (flags (@witx bitflags u32) $b $c))");
        let d = def_type("d", "(typename $d (flags (@witx bitflags u32) $b $c))");

        assert_eq!(a.representable(&d), RepEquality::Eq);
        assert_eq!(d.representable(&a), RepEquality::Eq);
    }

    #[test]
    fn enum_() {
        let base = def_type("a", "(typename $a (enum $b $c))");
        let extra_variant = def_type("a", "(typename $a (enum $b $c $d))");

        assert_eq!(base.representable(&extra_variant), RepEquality::Superset);
        assert_eq!(extra_variant.representable(&base), RepEquality::NotEq);

        let smaller_size = def_type("a", "(typename $a (enum (@witx tag u16) $b $c))");
        assert_eq!(smaller_size.representable(&base), RepEquality::Superset);
        assert_eq!(
            smaller_size.representable(&extra_variant),
            RepEquality::Superset
        );
    }

    #[test]
    fn union() {
        let base = def_type(
            "a",
            "(typename $tag (enum (@witx tag u8) $b $c))
            (typename $a (union (@witx tag $tag) u32 f32))",
        );
        let extra_variant = def_type(
            "a",
            "(typename $tag (enum (@witx tag u8) $b $c $d))
            (typename $a (union (@witx tag $tag) u32 f32 f64))",
        );

        assert_eq!(base.representable(&extra_variant), RepEquality::Superset);
        assert_eq!(extra_variant.representable(&base), RepEquality::NotEq);

        let other_ordering = def_type(
            "a",
            "(typename $tag (enum (@witx tag u8) $b $c))
            (typename $a (variant (@witx tag $tag) (case $c f32) (case $b u32)))",
        );
        assert_eq!(base.representable(&other_ordering), RepEquality::Eq);
        assert_eq!(other_ordering.representable(&base), RepEquality::Eq);
        assert_eq!(
            other_ordering.representable(&extra_variant),
            RepEquality::Superset
        );
    }
}
