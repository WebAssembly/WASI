use crate::{
    BuiltinType, EnumDatatype, FlagsDatatype, HandleDatatype, IntRepr, StructDatatype, Type,
    TypeRef, UnionDatatype,
};

pub trait Representable {
    fn representable(&self, by: &Self) -> bool;
    // XXX its not enough for this to be a bool. we need to give the recipe with which to represent
    // it -does it correspond exactly, does it need an upcast, or does it not correspond at all?
    // and so on for each member.
    // for structs, one might be able to represent each other, but not in the same memory layout.
    // this can be arbitrarily deep due to recursion. for ABI compatibility we may need
    // structs to correspond exactly, but maybe builtintypes just need to be representable. for
    // polyfilling, we may just need everything to be representable.
    // so, really this should return an enum describing what sort of equality we found between
    // the two types, and then let the caller make that policy decision. TODO: design exactly that
    // enum, i guess?
    // also what about equality of typeref?
}

impl Representable for BuiltinType {
    fn representable(&self, by: &Self) -> bool {
        // An unsigned integer can be used to represent an unsigned integer of smaller width.
        // Otherwise, types must be equal.
        match self {
            BuiltinType::U8 => match by {
                BuiltinType::U64 | BuiltinType::U32 | BuiltinType::U16 | BuiltinType::U8 => true,
                _ => false,
            },
            BuiltinType::U16 => match by {
                BuiltinType::U64 | BuiltinType::U32 | BuiltinType::U16 => true,
                _ => false,
            },
            BuiltinType::U32 => match by {
                BuiltinType::U64 | BuiltinType::U32 => true,
                _ => false,
            },
            other => by == other,
        }
    }
}

impl Representable for IntRepr {
    fn representable(&self, by: &Self) -> bool {
        // An unsigned integer can be used to represent an unsigned integer of smaller width.
        match self {
            IntRepr::U8 => true,
            IntRepr::U16 => match by {
                IntRepr::U16 | IntRepr::U32 | IntRepr::U64 => true,
                _ => false,
            },
            IntRepr::U32 => match by {
                IntRepr::U32 | IntRepr::U64 => true,
                _ => false,
            },
            IntRepr::U64 => *by == IntRepr::U64,
        }
    }
}

impl Representable for EnumDatatype {
    fn representable(&self, by: &Self) -> bool {
        // Integer representation must be compatible
        if !by.repr.representable(&self.repr) {
            return false;
        }
        // For each variant in self, must have variant of same name and position in by:
        for (ix, v) in self.variants.iter().enumerate() {
            if let Some(by_v) = by.variants.get(ix) {
                if by_v.name != v.name {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl Representable for FlagsDatatype {
    fn representable(&self, by: &Self) -> bool {
        // Integer representation must be compatible
        if !by.repr.representable(&self.repr) {
            return false;
        }
        // For each flag in self, must have flag of same name and position in by:
        for (ix, f) in self.flags.iter().enumerate() {
            if let Some(by_f) = by.flags.get(ix) {
                if by_f.name != f.name {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl Representable for HandleDatatype {
    fn representable(&self, by: &Self) -> bool {
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
                        if !nt.dt.representable(&by_nt.dt) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                TypeRef::Value(_) => {
                    return false;
                }
            }
        }
        true
    }
}

impl Representable for StructDatatype {
    fn representable(&self, _by: &Self) -> bool {
        unimplemented!(
            "this one is hard - need more than a bool for this return type to really describe it"
        )
    }
}

impl Representable for UnionDatatype {
    fn representable(&self, _by: &Self) -> bool {
        unimplemented!("this one is hard")
    }
}

impl Representable for TypeRef {
    fn representable(&self, _by: &Self) -> bool {
        unimplemented!("this one is hard - representable by type_() is appropriate in some cases, some times you may want precise name equality as well")
    }
}

impl Representable for Type {
    fn representable(&self, by: &Self) -> bool {
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
            _ => false,
        }
    }
}
