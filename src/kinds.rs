#[macro_export]
macro_rules! ptr {
    ($e: expr) => {{
        use crate::kinds::Kind;
        static E: &'static dyn Kind = &$e;
        static K: &'static dyn Kind = &crate::kinds::Ptr(E);
        K
    }};
}
#[macro_export]
macro_rules! array {
    ($e: expr, $c: expr) => {{
        use crate::kinds::Kind;
        static E: &'static dyn Kind = &$e;
        static K: &'static dyn Kind = &crate::kinds::Array(E, $c);
        K
    }};
}

#[allow(unused)]
pub trait Kind: Sync {
    fn name(&self) -> &'static str;
    fn short_name(&self) -> &'static str;
    fn size(&self) -> usize;
    fn pointee(&self) -> Option<&'static dyn Kind>;
}
impl<T: Kind> Kind for &T {
    fn name(&self) -> &'static str {
        (*self).name()
    }
    fn short_name(&self) -> &'static str {
        (*self).short_name()
    }
    fn size(&self) -> usize {
        (*self).size()
    }
    fn pointee(&self) -> Option<&'static dyn Kind> {
        (*self).pointee()
    }
}
impl Kind for &'static dyn Kind {
    fn name(&self) -> &'static str {
        (*self).name()
    }
    fn short_name(&self) -> &'static str {
        (*self).short_name()
    }
    fn size(&self) -> usize {
        (*self).size()
    }
    fn pointee(&self) -> Option<&'static dyn Kind> {
        (*self).pointee()
    }
}

#[allow(unused)]
pub struct Prim {
    pub name: &'static str,
    pub size: usize,
}
impl Kind for Prim {
    fn name(&self) -> &'static str {
        self.name
    }
    fn short_name(&self) -> &'static str {
        self.name
    }
    fn size(&self) -> usize {
        self.size
    }
    fn pointee(&self) -> Option<&'static dyn Kind> {
        None
    }
}

pub static VOID: &dyn Kind = &Prim {
    name: "void",
    size: 0,
};
pub static CHAR: &dyn Kind = &Prim {
    name: "char",
    size: 1,
};
pub static U8: &dyn Kind = &Prim {
    name: "u8",
    size: 1,
};
#[allow(unused)]
pub static S8: &dyn Kind = &Prim {
    name: "s8",
    size: 1,
};
pub static U16: &dyn Kind = &Prim {
    name: "u16",
    size: 2,
};
pub static S16: &dyn Kind = &Prim {
    name: "s16",
    size: 2,
};
pub static U32: &dyn Kind = &Prim {
    name: "u32",
    size: 4,
};
pub static S32: &dyn Kind = &Prim {
    name: "s32",
    size: 4,
};
pub static F32: &dyn Kind = &Prim {
    name: "f32",
    size: 4,
};

#[allow(unused)]
pub struct Array(pub &'static dyn Kind, pub usize);

impl Kind for Array {
    fn name(&self) -> &'static str {
        "[]"
    }
    fn short_name(&self) -> &'static str {
        self.name()
    }
    fn size(&self) -> usize {
        self.0.size() * self.1
    }
    fn pointee(&self) -> Option<&'static dyn Kind> {
        Some(self.0)
    }
}
#[allow(unused)]
pub struct Ptr(pub &'static dyn Kind);

impl Kind for Ptr {
    fn name(&self) -> &'static str {
        "*"
    }
    fn short_name(&self) -> &'static str {
        self.name()
    }
    fn size(&self) -> usize {
        4
    }
    fn pointee(&self) -> Option<&'static dyn Kind> {
        Some(self.0)
    }
}

pub struct Struct {
    pub name: &'static str,
    pub short_name: &'static str,
    pub fields: &'static [Field],
}
/// offset size name
pub struct Field(pub usize, pub &'static dyn Kind, pub &'static str);

impl Kind for Struct {
    fn name(&self) -> &'static str {
        self.name
    }
    fn short_name(&self) -> &'static str {
        self.short_name
    }
    fn size(&self) -> usize {
        self.size()
    }
    fn pointee(&self) -> Option<&'static dyn Kind> {
        None
    }
}

impl Struct {
    #[allow(unused)]
    pub fn check_offsets(&self) -> bool {
        let mut addr = 0;
        let mut i = 0;
        while i < self.fields.len() {
            let f = &self.fields[i];
            eprintln!("{}", f.2);
            if f.0 != addr {
                return false;
            }
            addr += f.1.size();
            i += 1;
        }
        true
    }
    pub fn size(&self) -> usize {
        let f = self.fields.last().unwrap();
        f.0 + f.1.size()
    }

    pub fn locate(&self, offset: usize) -> (&'static Field, usize) {
        assert!(offset < self.size());

        for f in self.fields {
            if offset >= f.0 + f.1.size() {
                continue;
            }
            return (f, offset - f.0);
        }
        unreachable!()
    }
}

pub fn name_of(mut kind: &dyn Kind) -> String {
    let mut name = String::new();
    let mut ptr_cnt = 0;

    loop {
        if let Some(inner) = kind.pointee() {
            ptr_cnt += 1;
            kind = inner;
        } else {
            name.push_str(kind.name());
            break;
        }
    }
    for _ in 0..ptr_cnt {
        name.push_str("*");
    }

    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!("u8", name_of(U8));
        assert_eq!("u8*", name_of(ptr!(U8)));
        assert_eq!("u8**", name_of(ptr!(ptr!(U8))));
    }
}
