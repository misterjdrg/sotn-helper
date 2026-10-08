use static_assertions::{const_assert, const_assert_eq};

pub struct Struct {
    pub name: &'static str,
    pub short_name: &'static str,
    pub fields: &'static [Field],
}
/// offset size name
pub struct Field(pub usize, pub usize, pub &'static str);

impl Struct {
    const fn check(&self) -> bool {
        let mut addr = 0;
        let mut i = 0;
        while i < self.fields.len() {
            let f = &self.fields[i];
            if f.0 != addr {
                return false;
            }
            addr += f.1;
            i += 1;
        }
        true
    }
    pub const fn size(&self) -> usize {
        let f = self.fields.last().unwrap();
        f.0 + f.1
    }

    pub fn locate(&self, offset: usize) -> (&'static Field, usize) {
        assert!(offset < self.size());

        for f in self.fields {
            if offset >= f.0 + f.1 {
                continue;
            }
            return (f, offset - f.0);
        }
        unreachable!()
    }
}

pub static ENTITY: Struct = Struct {
    name: "Entity",
    short_name: "entity",
    fields: &[
        Field(0x00, 4, "posX"),
        Field(0x04, 4, "posY"),
        Field(0x08, 4, "velocityX"),
        Field(0x0C, 4, "velocityY"),
        Field(0x10, 2, "hitboxOffX"),
        Field(0x12, 2, "hitboxOffY"),
        Field(0x14, 2, "facingLeft"),
        Field(0x16, 2, "palette"),
        Field(0x18, 1, "blendMode"),
        Field(0x19, 1, "drawFlags"),
        Field(0x1A, 2, "scaleX"),
        Field(0x1C, 2, "scaleY"),
        Field(0x1E, 2, "rotate"),
        Field(0x20, 2, "rotPivotX"),
        Field(0x22, 2, "rotPivotY"),
        Field(0x24, 2, "zPriority"),
        Field(0x26, 2, "entityId"),
        Field(0x28, 4, "pfnUpdate"),
        Field(0x2C, 2, "step"),
        Field(0x2E, 2, "step_s"),
        Field(0x30, 2, "params"),
        Field(0x32, 2, "entityRoomIndex"),
        Field(0x34, 4, "flags"),
        Field(0x38, 2, "__padding38__"),
        Field(0x3A, 2, "enemyId"),
        Field(0x3C, 2, "hitboxState"),
        Field(0x3E, 2, "hitPoints"),
        Field(0x40, 2, "attack"),
        Field(0x42, 2, "attackElement"),
        Field(0x44, 2, "hitParams"),
        Field(0x46, 1, "hitboxWidth"),
        Field(0x47, 1, "hitboxHeight"),
        Field(0x48, 1, "hitFlags"),
        Field(0x49, 1, "nFramesInvincibility"),
        Field(0x4A, 2, "unk4A"),
        Field(0x4C, 4, "anim"),
        Field(0x50, 2, "pose"),
        Field(0x52, 2, "poseTimer"),
        Field(0x54, 2, "animSet"),
        Field(0x56, 2, "animCurFrame"),
        Field(0x58, 2, "stunFrames"),
        Field(0x5A, 2, "unk5A"),
        Field(0x5C, 4, "parent"),
        Field(0x60, 4, "nextPart"),
        Field(0x64, 4, "primIndex"),
        Field(0x68, 2, "unk68"),
        Field(0x6A, 2, "hitEffect"),
        Field(0x6C, 1, "opacity"),
        Field(0x6D, 1, "unk6D[0]"),
        Field(0x6E, 1, "unk6D[1]"),
        Field(0x6F, 1, "unk6D[2]"),
        Field(0x70, 1, "unk6D[3]"),
        Field(0x71, 1, "unk6D[4]"),
        Field(0x72, 1, "unk6D[5]"),
        Field(0x73, 1, "unk6D[6]"),
        Field(0x74, 1, "unk6D[7]"),
        Field(0x75, 1, "unk6D[8]"),
        Field(0x76, 1, "unk6D[9]"),
        Field(0x77, 1, "unk6D[10]"),
        Field(0x78, 4, "unk78"),
        Field(0x7C, 60, "ext"),
        Field(0xB8, 4, "unkB8"),
    ],
};
const_assert!(ENTITY.check());
const_assert_eq!(0xBC, ENTITY.size());

pub static PRIMITIVE: Struct = Struct {
    name: "Primitive",
    short_name: "prim",
    fields: &[
        Field(0x00, 2, "type"),
        Field(0x02, 2, "priority"),
        Field(0x04, 2, "unk4"),
        Field(0x06, 2, "unk6"),
        Field(0x08, 2, "unk8"),
        Field(0x0A, 2, "unkA"),
        Field(0x0C, 2, "x0"),
        Field(0x0E, 2, "y0"),
        Field(0x10, 2, "x1"),
        Field(0x12, 2, "y1"),
        Field(0x14, 2, "x2"),
        Field(0x16, 2, "y2"),
        Field(0x18, 2, "x3"),
        Field(0x1A, 2, "y3"),
        Field(0x1C, 2, "unk1C"),
        Field(0x1E, 2, "drawMode"),
        Field(0x20, 4, "next"),
    ],
};
const_assert!(PRIMITIVE.check());
const_assert_eq!(0x24, PRIMITIVE.size());

pub static ENEMY_DEF: Struct = Struct {
    name: "EnemyDef",
    short_name: "enemy_def",
    fields: &[
        Field(0x00, 4, "name"),
        Field(0x04, 2, "hitPoints"),
        Field(0x06, 2, "attack"),
        Field(0x08, 2, "attackElement"),
        Field(0x0A, 2, "defense"),
        Field(0x0C, 2, "hitboxState"),
        Field(0x0E, 2, "weaknesses"),
        Field(0x10, 2, "strengths"),
        Field(0x12, 2, "immunes"),
        Field(0x14, 2, "absorbs"),
        Field(0x16, 2, "level"),
        Field(0x18, 2, "exp"),
        Field(0x1A, 2, "rareItemId"),
        Field(0x1C, 2, "uncommonItemId"),
        Field(0x1E, 2, "rareItemDropRate"),
        Field(0x20, 2, "uncommonItemDropRate"),
        Field(0x22, 1, "hitboxWidth"),
        Field(0x23, 1, "hitboxHeight"),
        Field(0x24, 4, "flags"),
    ],
};
const_assert!(ENEMY_DEF.check());
const_assert_eq!(0x28, ENEMY_DEF.size());
