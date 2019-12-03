use crate::{
    BuiltinType, EnumDatatype, FlagsDatatype, HandleDatatype, IntRepr, NamedType, StructDatatype,
    Type, TypeRef, UnionDatatype,
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

impl Representable for HandleDatatype {
    fn representable(&self, by: &Self) -> RepEquality {
        // Handles must have the same set of named supertypes. Anonymous supertypes are never
        // equal, and the validator should probably make sure these are not allowed, because
        // what would that even mean??
        for supertype_ref in self.supertypes.iter() {
            match supertype_ref {
                TypeRef::Name(nt) => {
                    if let Some(by_nt) = by.supertypes.iter().find_map(|tref| match tref {
                        TypeRef::Name(by_nt) if by_nt.name == nt.name => Some(by_nt),
                        _ => None,
                    }) {
                        if nt.tref.representable(&by_nt.tref) == RepEquality::NotEq {
                            return RepEquality::NotEq;
                        }
                    } else {
                        return RepEquality::NotEq;
                    }
                }
                TypeRef::Value(_) => {
                    return RepEquality::NotEq;
                }
            }
        }
        RepEquality::Eq
    }
}

impl Representable for StructDatatype {
    fn representable(&self, by: &Self) -> RepEquality {
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
        if self.variants.len() > by.variants.len() {
            return RepEquality::NotEq;
        }
        for (v, byv) in self.variants.iter().zip(by.variants.iter()) {
            if v.name != byv.name {
                return RepEquality::NotEq;
            }
            if v.tref.type_().representable(&*byv.tref.type_()) != RepEquality::Eq {
                return RepEquality::NotEq;
            }
        }
        if by.variants.len() > self.variants.len() {
            RepEquality::Superset
        } else {
            RepEquality::Eq
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
            (Type::Handle(s), Type::Handle(b)) => s.representable(b),
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
    fn different_typenames_equal() {
        let a = def_type("a", "(typename $a (flags u32 $b $c))");
        let d = def_type("d", "(typename $d (flags u32 $b $c))");

        assert_eq!(a.representable(&d), RepEquality::Eq);
        assert_eq!(d.representable(&a), RepEquality::Eq);
    }

    #[test]
    fn different_flagnames_not_equal() {
        let a = def_type("a", "(typename $a (flags u32 $b $c))");
        let d = def_type("d", "(typename $d (flags u32 $b $e))");

        assert_eq!(a.representable(&d), RepEquality::NotEq);
        assert_eq!(d.representable(&a), RepEquality::NotEq);
    }

    #[test]
    fn flag_superset() {
        let base = def_type("a", "(typename $a (flags u32 $b $c))");
        let extra_flag = def_type("a", "(typename $a (flags u32 $b $c $d))");

        assert_eq!(base.representable(&extra_flag), RepEquality::Superset);
        assert_eq!(extra_flag.representable(&base), RepEquality::NotEq);

        let smaller_size = def_type("a", "(typename $a (flags u16 $b $c))");
        assert_eq!(smaller_size.representable(&base), RepEquality::Superset);
        assert_eq!(
            smaller_size.representable(&extra_flag),
            RepEquality::Superset
        );
        assert_eq!(base.representable(&smaller_size), RepEquality::NotEq);
    }

    #[test]
    fn enum_superset() {
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
    fn union_superset() {
        let base = def_type("a", "(typename $a (union (field $b u32) (field $c f32)))");
        let extra_variant = def_type(
            "a",
            "(typename $a (union (field $b u32) (field $c f32) (field $d f64)))",
        );

        assert_eq!(base.representable(&extra_variant), RepEquality::Superset);
        assert_eq!(extra_variant.representable(&base), RepEquality::NotEq);
    }
}
