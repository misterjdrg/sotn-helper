# sotn-helper

## Usage
### Offset
Get field name (and pointer offset if apropriate)

```bash
# with entity struct
> sotn_helper offset entity unk8
self->velocityX


> sotn_helper offset entity unk100
entity = self + 1;
entity->hitParams


> sotn_helper offset entity unk-a8
entity = self - 1;
entity->facingLeft


> sotn_helper offset entity unk-10
entity = self - 1;
entity->ext.ILLEGAL.u8[48]


> sotn_helper offset entity "custom_entity->unk-a8"
entity = custom_entity - 1;
entity->facingLeft


# with different structs
> sotn_helper offset prim unk108
primitive = self + 7;
primitive->x0


> sotn_helper offset enemydef unk-4
enemydef = self - 1;
enemydef->flags
```
