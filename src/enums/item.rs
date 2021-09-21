use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
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
    DarkOakPlanks = 27,
    CrimsonPlanks = 28,
    WarpedPlanks = 29,
    OakSapling = 30,
    SpruceSapling = 31,
    BirchSapling = 32,
    JungleSapling = 33,
    AcaciaSapling = 34,
    DarkOakSapling = 35,
    Bedrock = 36,
    Sand = 37,
    RedSand = 38,
    Gravel = 39,
    CoalOre = 40,
    DeepslateCoalOre = 41,
    IronOre = 42,
    DeepslateIronOre = 43,
    CopperOre = 44,
    DeepslateCopperOre = 45,
    GoldOre = 46,
    DeepslateGoldOre = 47,
    RedstoneOre = 48,
    DeepslateRedstoneOre = 49,
    EmeraldOre = 50,
    DeepslateEmeraldOre = 51,
    LapisOre = 52,
    DeepslateLapisOre = 53,
    DiamondOre = 54,
    DeepslateDiamondOre = 55,
    NetherGoldOre = 56,
    NetherQuartzOre = 57,
    AncientDebris = 58,
    CoalBlock = 59,
    RawIronBlock = 60,
    RawCopperBlock = 61,
    RawGoldBlock = 62,
    AmethystBlock = 63,
    BuddingAmethyst = 64,
    IronBlock = 65,
    CopperBlock = 66,
    GoldBlock = 67,
    DiamondBlock = 68,
    NetheriteBlock = 69,
    ExposedCopper = 70,
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
        return num_traits::FromPrimitive::from_i32(value.value()).ok_or(Error::EnumOutOfBound);
    }
}

