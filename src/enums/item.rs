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
    DripstoneBlock = 13,
    GrassBlock = 14,
    Dirt = 15,
    CoarseDirt = 16,
    Podzol = 17,
    RootedDirt = 18,
    CrimsonNylium = 19,
    WarpedNylium = 20,
    Cobblestone = 21,
    OakPlanks = 22,
    SprucePlanks = 23,
    BirchPlanks = 24,
    JunglePlanks = 25,
    AcaciaPlanks = 26,
}

impl Item {
    pub fn to_identifier(self) -> crate::Identifier {
        todo!();
    }
}

use crate::Error;

impl TryFrom<crate::VarInt> for Item {
    type Error = Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        match value.value() {
            x if x == Self::Air as i32 => Ok(Self::Air),
            x if x == Self::Stone as i32 => Ok(Self::Stone),
            x if x == Self::Granite as i32 => Ok(Self::Granite),
            x if x == Self::PolishedGranite as i32 => Ok(Self::PolishedGranite),
            x if x == Self::Diorite as i32 => Ok(Self::Diorite),
            x if x == Self::PolishedDiorite as i32 => Ok(Self::PolishedDiorite),
            x if x == Self::Andesite as i32 => Ok(Self::Andesite),
            x if x == Self::PolishedAndesite as i32 => Ok(Self::PolishedAndesite),
            x if x == Self::Deepslate as i32 => Ok(Self::Deepslate),
            x if x == Self::CobbledDeepslate as i32 => Ok(Self::CobbledDeepslate),
            x if x == Self::PolishedDeepslate as i32 => Ok(Self::PolishedDeepslate),
            x if x == Self::Calcite as i32 => Ok(Self::Calcite),
            x if x == Self::Tuff as i32 => Ok(Self::Tuff),
            x if x == Self::DripstoneBlock as i32 => Ok(Self::DripstoneBlock),
            x if x == Self::GrassBlock as i32 => Ok(Self::GrassBlock),
            x if x == Self::Dirt as i32 => Ok(Self::Dirt),
            x if x == Self::CoarseDirt as i32 => Ok(Self::CoarseDirt),
            x if x == Self::Podzol as i32 => Ok(Self::Podzol),
            x if x == Self::RootedDirt as i32 => Ok(Self::RootedDirt),
            x if x == Self::CrimsonNylium as i32 => Ok(Self::CrimsonNylium),
            x if x == Self::WarpedNylium as i32 => Ok(Self::WarpedNylium),
            x if x == Self::Cobblestone as i32 => Ok(Self::Cobblestone),
            x if x == Self::OakPlanks as i32 => Ok(Self::OakPlanks),
            x if x == Self::SprucePlanks as i32 => Ok(Self::SprucePlanks),
            x if x == Self::BirchPlanks as i32 => Ok(Self::BirchPlanks),
            x if x == Self::JunglePlanks as i32 => Ok(Self::JunglePlanks),
            x if x == Self::AcaciaPlanks as i32 => Ok(Self::AcaciaPlanks),
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

