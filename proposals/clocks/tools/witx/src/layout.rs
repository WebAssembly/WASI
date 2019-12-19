use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SizeAlign {
    pub size: usize,
    pub align: usize,
}

pub trait Layout {
    fn mem_size_align(&self) -> SizeAlign;
    fn mem_size(&self) -> usize {
        self.mem_size_align().size
    }
    fn mem_align(&self) -> usize {
        self.mem_size_align().align
    }
}

impl TypeRef {
    fn layout(&self, cache: &mut HashMap<TypeRef, SizeAlign>) -> SizeAlign {
        if let Some(hit) = cache.get(self) {
            return *hit;
        }
        let layout = match &*self.type_() {
            Type::Enum(e) => e.repr.layout(),
            Type::Flags(f) => f.repr.layout(),
            Type::Struct(s) => s.layout(cache),
            Type::Union(u) => u.layout(cache),
            Type::Handle { .. } => BuiltinType::U32.layout(),
            Type::Array { .. } => BuiltinType::String.layout(),
            Type::Pointer { .. } | Type::ConstPointer { .. } => BuiltinType::U32.layout(),
            Type::Builtin(b) => b.layout(),
        };
        cache.insert(self.clone(), layout);
        layout
    }
}

impl Layout for TypeRef {
    fn mem_size_align(&self) -> SizeAlign {
        let mut cache = HashMap::new();
        self.layout(&mut cache)
    }
}

impl IntRepr {
    pub fn layout(&self) -> SizeAlign {
        match self {
            IntRepr::U8 => BuiltinType::U8.layout(),
            IntRepr::U16 => BuiltinType::U16.layout(),
            IntRepr::U32 => BuiltinType::U32.layout(),
            IntRepr::U64 => BuiltinType::U64.layout(),
        }
    }
}

pub struct StructMemberLayout<'a> {
    member: &'a StructMember,
    offset: usize,
}

impl StructDatatype {
    pub fn member_layout(
        &self,
        cache: &mut HashMap<TypeRef, SizeAlign>,
    ) -> Vec<StructMemberLayout> {
        let mut members = Vec::new();
        let mut offset = 0;
        for m in self.members.iter() {
            let sa = m.tref.layout(cache);
            offset = align_to(offset, sa.align);
            members.push(StructMemberLayout { member: m, offset });
            offset += sa.size;
        }
        members
    }

    pub fn layout(&self, cache: &mut HashMap<TypeRef, SizeAlign>) -> SizeAlign {
        let members = self.member_layout(cache);
        let align = members
            .iter()
            .map(|m| m.member.tref.layout(cache).align)
            .max()
            .expect("nonzero struct members");
        let last_offset = members.last().expect("nonzero struct members").offset;
        let size = align_to(last_offset, align);
        SizeAlign { size, align }
    }
}

/// If the next free byte in the struct is `offs`, and the next
/// element has alignment `alignment`, determine the offset at
/// which to place that element.
fn align_to(offs: usize, alignment: usize) -> usize {
    offs + alignment - 1 - ((offs + alignment - 1) % alignment)
}

#[cfg(test)]
mod test {
    use super::align_to;
    #[test]
    fn align() {
        assert_eq!(0, align_to(0, 1));
        assert_eq!(0, align_to(0, 2));
        assert_eq!(0, align_to(0, 4));
        assert_eq!(0, align_to(0, 8));

        assert_eq!(1, align_to(1, 1));
        assert_eq!(2, align_to(1, 2));
        assert_eq!(4, align_to(1, 4));
        assert_eq!(8, align_to(1, 8));

        assert_eq!(2, align_to(2, 1));
        assert_eq!(2, align_to(2, 2));
        assert_eq!(4, align_to(2, 4));
        assert_eq!(8, align_to(2, 8));

        assert_eq!(5, align_to(5, 1));
        assert_eq!(6, align_to(5, 2));
        assert_eq!(8, align_to(5, 4));
        assert_eq!(8, align_to(5, 8));
    }
}

impl UnionDatatype {
    pub fn layout(&self, cache: &mut HashMap<TypeRef, SizeAlign>) -> SizeAlign {
        let sas = self
            .variants
            .iter()
            .map(|v| v.tref.layout(cache))
            .collect::<Vec<SizeAlign>>();
        let size = sas
            .iter()
            .map(|sa| sa.size)
            .max()
            .expect("nonzero variants");
        let align = sas
            .iter()
            .map(|sa| sa.align)
            .max()
            .expect("nonzero variants");
        SizeAlign { size, align }
    }
}

impl BuiltinType {
    pub fn layout(&self) -> SizeAlign {
        match self {
            BuiltinType::String => SizeAlign { size: 8, align: 4 }, // Pointer and Length
            BuiltinType::U8 | BuiltinType::S8 => SizeAlign { size: 1, align: 1 },
            BuiltinType::U16 | BuiltinType::S16 => SizeAlign { size: 2, align: 2 },
            BuiltinType::U32 | BuiltinType::S32 | BuiltinType::F32 => {
                SizeAlign { size: 4, align: 4 }
            }
            BuiltinType::U64 | BuiltinType::S64 | BuiltinType::F64 => {
                SizeAlign { size: 8, align: 8 }
            }
        }
    }
}
