macro_rules! ptr {
    ($e: expr) => {
        &Ptr(&$e)
    };
}

pub static ENTITY: Struct = Struct {
    name: "Entity",
    short_name: "entity",
    fields: &[
        Field(0x00, F32, "posX"),
        Field(0x04, F32, "posY"),
        Field(0x08, S32, "velocityX"),
        Field(0x0C, S32, "velocityY"),
        Field(0x10, S16, "hitboxOffX"),
        Field(0x12, S16, "hitboxOffY"),
        Field(0x14, U16, "facingLeft"),
        Field(0x16, U16, "palette"),
        Field(0x18, U8, "blendMode"),
        Field(0x19, U8, "drawFlags"),
        Field(0x1A, S16, "scaleX"),
        Field(0x1C, S16, "scaleY"),
        Field(0x1E, S16, "rotate"),
        Field(0x20, S16, "rotPivotX"),
        Field(0x22, S16, "rotPivotY"),
        Field(0x24, U16, "zPriority"),
        Field(0x26, U16, "entityId"),
        Field(0x28, ptr!(VOID), "pfnUpdate"),
        Field(0x2C, U16, "step"),
        Field(0x2E, U16, "step_s"),
        Field(0x30, U16, "params"),
        Field(0x32, U16, "entityRoomIndex"),
        Field(0x34, S32, "flags"),
        Field(0x38, S16, "__padding38__"),
        Field(0x3A, U16, "enemyId"),
        Field(0x3C, U16, "hitboxState"),
        Field(0x3E, S16, "hitPoints"),
        Field(0x40, S16, "attack"),
        Field(0x42, U16, "attackElement"),
        Field(0x44, U16, "hitParams"),
        Field(0x46, U8, "hitboxWidth"),
        Field(0x47, U8, "hitboxHeight"),
        Field(0x48, U8, "hitFlags"),
        Field(0x49, U8, "nFramesInvincibility"),
        Field(0x4A, U16, "unk4A"),
        Field(0x4C, ptr!(VOID), "anim"),
        Field(0x50, U16, "pose"),
        Field(0x52, S16, "poseTimer"),
        Field(0x54, S16, "animSet"),
        Field(0x56, S16, "animCurFrame"),
        Field(0x58, S16, "stunFrames"),
        Field(0x5A, U16, "unk5A"),
        Field(0x5C, ptr!(ENTITY), "parent"),
        Field(0x60, ptr!(VOID), "nextPart"),
        Field(0x64, U32, "primIndex"),
        Field(0x68, U16, "unk68"),
        Field(0x6A, U16, "hitEffect"),
        Field(0x6C, U8, "opacity"),
        Field(0x6D, U8, "unk6D[0]"),
        Field(0x6E, U8, "unk6D[1]"),
        Field(0x6F, U8, "unk6D[2]"),
        Field(0x70, U8, "unk6D[3]"),
        Field(0x71, U8, "unk6D[4]"),
        Field(0x72, U8, "unk6D[5]"),
        Field(0x73, U8, "unk6D[6]"),
        Field(0x74, U8, "unk6D[7]"),
        Field(0x75, U8, "unk6D[8]"),
        Field(0x76, U8, "unk6D[9]"),
        Field(0x77, U8, "unk6D[10]"),
        Field(0x78, S32, "unk78"),
        Field(0x7C, &EntityExt, "ext"),
        Field(0xB8, ptr!(ENTITY), "unkB8"),
    ],
};
pub static PRIMITIVE: Struct = Struct {
    name: "Primitive",
    short_name: "prim",
    fields: &[
        Field(0x00, ptr!(PRIMITIVE), "next"),
        Field(0x04, U8, "r0"),
        Field(0x05, U8, "g0"),
        Field(0x06, U8, "b0"),
        Field(0x07, U8, "type"),
        Field(0x08, S16, "x0"),
        Field(0x0A, S16, "y0"),
        Field(0x0C, U8, "u0"),
        Field(0x0D, U8, "v0"),
        Field(0x0E, U16, "clut"),
        Field(0x10, U8, "r1"),
        Field(0x11, U8, "g1"),
        Field(0x12, U8, "b1"),
        Field(0x13, U8, "p1"),
        Field(0x14, S16, "x1"),
        Field(0x16, S16, "y1"),
        Field(0x18, U8, "u1"),
        Field(0x19, U8, "v1"),
        Field(0x1A, U16, "tpage"),
        Field(0x1C, U8, "r2"),
        Field(0x1D, U8, "g2"),
        Field(0x1E, U8, "b2"),
        Field(0x1F, U8, "p2"),
        Field(0x20, S16, "x2"),
        Field(0x22, S16, "y2"),
        Field(0x24, U8, "u2"),
        Field(0x25, U8, "v2"),
        Field(0x26, U16, "priority"),
        Field(0x28, U8, "r3"),
        Field(0x29, U8, "g3"),
        Field(0x2A, U8, "b3"),
        Field(0x2B, U8, "p3"),
        Field(0x2C, S16, "x3"),
        Field(0x2E, S16, "y3"),
        Field(0x30, U8, "u3"),
        Field(0x31, U8, "v3"),
        Field(0x32, U16, "drawMode"),
    ],
};

pub static ENEMY_DEF: Struct = Struct {
    name: "EnemyDef",
    short_name: "enemy_def",
    fields: &[
        Field(0x00, ptr!(CHAR), "name"),
        Field(0x04, S16, "hitPoints"),
        Field(0x06, S16, "attack"),
        Field(0x08, U16, "attackElement"),
        Field(0x0A, S16, "defense"),
        Field(0x0C, U16, "hitboxState"),
        Field(0x0E, U16, "weaknesses"),
        Field(0x10, U16, "strengths"),
        Field(0x12, U16, "immunes"),
        Field(0x14, U16, "absorbs"),
        Field(0x16, U16, "level"),
        Field(0x18, U16, "exp"),
        Field(0x1A, U16, "rareItemId"),
        Field(0x1C, U16, "uncommonItemId"),
        Field(0x1E, U16, "rareItemDropRate"),
        Field(0x20, U16, "uncommonItemDropRate"),
        Field(0x22, U8, "hitboxWidth"),
        Field(0x23, U8, "hitboxHeight"),
        Field(0x24, S32, "flags"),
    ],
};

#[allow(unused)]
pub trait Kind: Sync {
    fn name(&self) -> &'static str;
    fn short_name(&self) -> &'static str;
    fn size(&self) -> usize;
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

pub struct EntityExt;
impl Kind for EntityExt {
    fn name(&self) -> &'static str {
        "u8[]"
    }
    fn short_name(&self) -> &'static str {
        self.name()
    }
    fn size(&self) -> usize {
        60
    }
}

#[allow(unused)]
pub struct Ptr(&'static dyn Kind);

impl Kind for Ptr {
    fn name(&self) -> &'static str {
        "ptr"
    }
    fn short_name(&self) -> &'static str {
        self.name()
    }
    fn size(&self) -> usize {
        4
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
}

impl Struct {
    #[allow(unused)]
    fn check(&self) -> bool {
        let mut addr = 0;
        let mut i = 0;
        while i < self.fields.len() {
            let f = &self.fields[i];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks() {
        assert!(ENTITY.check());
        assert_eq!(0xBC, ENTITY.size());

        assert!(PRIMITIVE.check());
        assert_eq!(0x34, PRIMITIVE.size());

        assert!(ENEMY_DEF.check());
        assert_eq!(0x28, ENEMY_DEF.size());
    }
}
