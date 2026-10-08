use crate::array;
use crate::kinds::{
    CHAR as char, F32 as f32, Field, S16 as s16, S32 as s32, Struct, U8 as u8, U16 as u16,
    U32 as u32, VOID as void,
};
use crate::ptr;

pub static ENTITY: Struct = Struct {
    name: "Entity",
    short_name: "entity",
    fields: &[
        Field(0x00, f32, "posX"),
        Field(0x04, f32, "posY"),
        Field(0x08, s32, "velocityX"),
        Field(0x0C, s32, "velocityY"),
        Field(0x10, s16, "hitboxOffX"),
        Field(0x12, s16, "hitboxOffY"),
        Field(0x14, u16, "facingLeft"),
        Field(0x16, u16, "palette"),
        Field(0x18, u8, "blendMode"),
        Field(0x19, u8, "drawFlags"),
        Field(0x1A, s16, "scaleX"),
        Field(0x1C, s16, "scaleY"),
        Field(0x1E, s16, "rotate"),
        Field(0x20, s16, "rotPivotX"),
        Field(0x22, s16, "rotPivotY"),
        Field(0x24, u16, "zPriority"),
        Field(0x26, u16, "entityId"),
        Field(0x28, ptr![void], "pfnUpdate"),
        Field(0x2C, u16, "step"),
        Field(0x2E, u16, "step_s"),
        Field(0x30, u16, "params"),
        Field(0x32, u16, "entityRoomIndex"),
        Field(0x34, s32, "flags"),
        Field(0x38, s16, "__padding38__"),
        Field(0x3A, u16, "enemyId"),
        Field(0x3C, u16, "hitboxState"),
        Field(0x3E, s16, "hitPoints"),
        Field(0x40, s16, "attack"),
        Field(0x42, u16, "attackElement"),
        Field(0x44, u16, "hitParams"),
        Field(0x46, u8, "hitboxWidth"),
        Field(0x47, u8, "hitboxHeight"),
        Field(0x48, u8, "hitFlags"),
        Field(0x49, u8, "nFramesInvincibility"),
        Field(0x4A, u16, "unk4A"),
        Field(0x4C, ptr![void], "anim"),
        Field(0x50, u16, "pose"),
        Field(0x52, s16, "poseTimer"),
        Field(0x54, s16, "animSet"),
        Field(0x56, s16, "animCurFrame"),
        Field(0x58, s16, "stunFrames"),
        Field(0x5A, u16, "unk5A"),
        Field(0x5C, ptr![ENTITY], "parent"),
        Field(0x60, ptr![void], "nextPart"),
        Field(0x64, u32, "primIndex"),
        Field(0x68, u16, "unk68"),
        Field(0x6A, u16, "hitEffect"),
        Field(0x6C, u8, "opacity"),
        Field(0x6D, array![u8, 11], "unk6D"),
        Field(0x78, s32, "unk78"),
        Field(0x7C, array![u8, 60], "ext"),
        Field(0xB8, ptr![ENTITY], "unkB8"),
    ],
};
pub static PRIMITIVE: Struct = Struct {
    name: "Primitive",
    short_name: "prim",
    fields: &[
        Field(0x00, ptr![PRIMITIVE], "next"),
        Field(0x04, u8, "r0"),
        Field(0x05, u8, "g0"),
        Field(0x06, u8, "b0"),
        Field(0x07, u8, "type"),
        Field(0x08, s16, "x0"),
        Field(0x0A, s16, "y0"),
        Field(0x0C, u8, "u0"),
        Field(0x0D, u8, "v0"),
        Field(0x0E, u16, "clut"),
        Field(0x10, u8, "r1"),
        Field(0x11, u8, "g1"),
        Field(0x12, u8, "b1"),
        Field(0x13, u8, "p1"),
        Field(0x14, s16, "x1"),
        Field(0x16, s16, "y1"),
        Field(0x18, u8, "u1"),
        Field(0x19, u8, "v1"),
        Field(0x1A, u16, "tpage"),
        Field(0x1C, u8, "r2"),
        Field(0x1D, u8, "g2"),
        Field(0x1E, u8, "b2"),
        Field(0x1F, u8, "p2"),
        Field(0x20, s16, "x2"),
        Field(0x22, s16, "y2"),
        Field(0x24, u8, "u2"),
        Field(0x25, u8, "v2"),
        Field(0x26, u16, "priority"),
        Field(0x28, u8, "r3"),
        Field(0x29, u8, "g3"),
        Field(0x2A, u8, "b3"),
        Field(0x2B, u8, "p3"),
        Field(0x2C, s16, "x3"),
        Field(0x2E, s16, "y3"),
        Field(0x30, u8, "u3"),
        Field(0x31, u8, "v3"),
        Field(0x32, u16, "drawMode"),
    ],
};

pub static ENEMY_DEF: Struct = Struct {
    name: "EnemyDef",
    short_name: "enemy_def",
    fields: &[
        Field(0x00, ptr![char], "name"),
        Field(0x04, s16, "hitPoints"),
        Field(0x06, s16, "attack"),
        Field(0x08, u16, "attackElement"),
        Field(0x0A, s16, "defense"),
        Field(0x0C, u16, "hitboxState"),
        Field(0x0E, u16, "weaknesses"),
        Field(0x10, u16, "strengths"),
        Field(0x12, u16, "immunes"),
        Field(0x14, u16, "absorbs"),
        Field(0x16, u16, "level"),
        Field(0x18, u16, "exp"),
        Field(0x1A, u16, "rareItemId"),
        Field(0x1C, u16, "uncommonItemId"),
        Field(0x1E, u16, "rareItemDropRate"),
        Field(0x20, u16, "uncommonItemDropRate"),
        Field(0x22, u8, "hitboxWidth"),
        Field(0x23, u8, "hitboxHeight"),
        Field(0x24, s32, "flags"),
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks() {
        assert!(ENTITY.check_offsets());
        assert_eq!(0xBC, ENTITY.size());

        assert!(PRIMITIVE.check_offsets());
        assert_eq!(0x34, PRIMITIVE.size());

        assert!(ENEMY_DEF.check_offsets());
        assert_eq!(0x28, ENEMY_DEF.size());
    }
}
