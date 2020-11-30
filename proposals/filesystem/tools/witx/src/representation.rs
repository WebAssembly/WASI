use crate::{
    BuiltinType, EnumDatatype, FlagsDatatype, IntRepr, NamedType, StructDatatype, Type, TypeRef,
    UnionDatatype,
};

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

impl Representable for EnumDatatype {
    fn representable(&self, by: &Self) -> RepEquality {
        // Integer representation must be compatible
        if self.repr.representable(&by.repr) == RepEquality::NotEq {
            return RepEquality::NotEq;
        }
        // For each variant in self, must have variant of same name and position in by:
        for (ix, v) in self.variants.iter().enumerate() {
            if let Some(by_v) = by.variants.get(ix) {
                if by_v.name != v.name {
                    return RepEquality::NotEq;
                }
            } else {
                return RepEquality::NotEq;
            }
        }
        if by.variants.len() > self.variants.len() {
            RepEquality::Superset
        } else {
            self.repr.representable(&by.repr)
        }
    }
}

impl Representable for FlagsDatatype {
    fn representable(&self, by: &Self) -> RepEquality {
        // Integer representation must be compatible
        if self.repr.representable(&by.repr) == RepEquality::NotEq {
            return RepEquality::NotEq;
        }
        // For each flag in self, must have flag of same name and position in by:
        for (ix, f) in self.flags.iter().enumerate() {
            if let Some(by_f) = by.flags.get(ix) {
                if by_f.name != f.name {
                    return RepEquality::NotEq;
                }
            } else {
                return RepEquality::NotEq;
            }
        }
        if by.flags.len() > self.flags.len() {
            RepEquality::Superset
        } else {
            self.repr.representable(&by.repr)
        }
    }
}

impl Representable for StructDatatype {
    fn representable(&self, by: &Self) -> RepEquality {
        // Structs must have exact structural equality - same members, must
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

impl Representable for UnionDatatype {
    fn representable(&self, by: &Self) -> RepEquality {
        // Unions must have equal variants, by name (independent of order). If `by` has extra
        // variants, its a superset.
        // We would require require a more expressive RepEquality enum to describe which variants
        // might be supersets.
        if self.variants.len() > by.variants.len() {
            return RepEquality::NotEq;
        }
        for v in self.variants.iter() {
            if let Some(byv) = by.variants.iter().find(|byv| byv.name == v.name) {
                if v.tref.is_none() && byv.tref.is_none() {
                    // Both empty is OK
                } else if v.tref.is_some() && byv.tref.is_some() {
                    if v.tref
                        .as_ref()
                        .unwrap()
                        .type_()
                        .representable(&*byv.tref.as_ref().unwrap().type_())
                        != RepEquality::Eq
                    {
                        // Fields must be Eq
                        return RepEquality::NotEq;
                    }
                } else {
                    // Either one empty means not representable
                    return RepEquality::NotEq;
                }
            } else {
                return RepEquality::NotEq;
            }
        }
        if by.variants.len() > self.variants.len() {
            // By is a superset of self only if the tags are as well:
            if self.tag.type_().representable(&*by.tag.type_()) == RepEquality::Superset {
                RepEquality::Superset
            } else {
                RepEquality::NotEq
            }
        } else {
            // By and self have matching variants, so they are equal if tags are:
            if self.tag.type_().representable(&*by.tag.type_()) == RepEquality::Eq {
                RepEquality::Eq
            } else {
                RepEquality::NotEq
            }
        }
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
            (Type::Enum(s), Type::Enum(b)) => s.representable(b),
            (Type::Flags(s), Type::Flags(b)) => s.representable(b),
            (Type::Struct(s), Type::Struct(b)) => s.representable(b),
            (Type::Union(s), Type::Union(b)) => s.representable(b),
            (Type::Handle(_), Type::Handle(_)) => RepEquality::Eq, // Handles are nominal, not structural
            (Type::Array(s), Type::Array(b)) => s.representable(b),
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
        let a = def_type("a", "(typename $a (flags u32 $b $c))");
        let d = def_type("d", "(typename $d (flags u32 $b $c))");

        assert_eq!(a.representable(&d), RepEquality::Eq);
        assert_eq!(d.representable(&a), RepEquality::Eq);
    }

    #[test]
    fn flags() {
        let base = def_type("a", "(typename $a (flags u32 $b $c))");
        let extra_flag = def_type("a", "(typename $a (flags u32 $b $c $d))");

        assert_eq!(base.representable(&extra_flag), RepEquality::Superset);
        assert_eq!(extra_flag.representable(&base), RepEquality::NotEq);

        let different_flagnames = def_type("d", "(typename $d (flags u32 $b $e))");
        assert_eq!(base.representable(&different_flagnames), RepEquality::NotEq);
        assert_eq!(different_flagnames.representable(&base), RepEquality::NotEq);

        let smaller_size = def_type("a", "(typename $a (flags u16 $b $c))");
        assert_eq!(smaller_size.representable(&base), RepEquality::Superset);
        assert_eq!(
            smaller_size.representable(&extra_flag),
            RepEquality::Superset
        );
        assert_eq!(base.representable(&smaller_size), RepEquality::NotEq);
    }

    #[test]
    fn enum_() {
        let base = def_type("a", "(typename $a (enum u32 $b $c))");
        let extra_variant = def_type("a", "(typename $a (enum u32 $b $c $d))");

        assert_eq!(base.representable(&extra_variant), RepEquality::Superset);
        assert_eq!(extra_variant.representable(&base), RepEquality::NotEq);

        let smaller_size = def_type("a", "(typename $a (enum u16 $b $c))");
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
            "(typename $tag (enum u8 $b $c))
            (typename $a (union $tag (field $b u32) (field $c f32)))",
        );
        let extra_variant = def_type(
            "a",
            "(typename $tag (enum u8 $b $c $d))
            (typename $a (union $tag (field $b u32) (field $c f32) (field $d f64)))",
        );

        assert_eq!(base.representable(&extra_variant), RepEquality::Superset);
        assert_eq!(extra_variant.representable(&base), RepEquality::NotEq);

        let other_ordering = def_type(
            "a",
            "(typename $tag (enum u8 $b $c))
            (typename $a (union $tag (field $c f32) (field $b u32)))",
        );
        assert_eq!(base.representable(&other_ordering), RepEquality::Eq);
        assert_eq!(other_ordering.representable(&base), RepEquality::Eq);
        assert_eq!(
            other_ordering.representable(&extra_variant),
            RepEquality::Superset
        );
    }
}
