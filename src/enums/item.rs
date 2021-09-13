use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
/// Represents a block. Not all blocks are implimented or supported yet.
pub enum Item {
    Air = 0,
    Stone = 1,
    Granite = 2,
    PolishedGranite = 3,
    Diorite = 4,
    PolishedDiorite = 5,
    Andesite = 6,
    PolishedAndesite = 7,
    Deepslate = 8,
    CobbledDeepslate = 9,
    PolishedDeepslate = 10,
    Calcite = 11,
    Tuff = 12,
    DripstoneBlock = 13
}

impl Item {
    pub fn to_identifier(self) -> crate::Identifier {
        todo!();
    }
}

impl TryFrom<crate::VarInt> for Item {
    type Error = crate::Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        match value {
            _ => {}
        }
        todo!();
    }
}

