use crate::Error;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Snowy {
    True,
    False
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaplingGrowthStage {
    Stage0,
    Stage1
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FluidLevel {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
    Level10,
    Level11,
    Level12,
    Level13,
    Level14,
    Level15
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Axis {
    X,
    Y,
    Z
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LeafDistance {
    Distance1,
    Distance2,
    Distance3,
    Distance4,
    Distance5,
    Distance6,
    Distance7
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LeafPersistence {
    True,
    False
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Triggered {
    True,
    False
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
/// Represents a block. Not all blocks are implimented or supported yet.
pub enum Block {
    Air,
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    GrassBlock(Snowy),
    Dirt,
    CoarseDirt,
    Podzol(Snowy),
    Cobblestone,
    OakPlanks,
    SprucePlanks,
    BirchPlanks,
    JunglePlanks,
    AcaciaPlanks,
    DarkOakPlanks,
    OakSapling(SaplingGrowthStage),
    SpruceSapling(SaplingGrowthStage),
    BirchSapling(SaplingGrowthStage),
    JungleSapling(SaplingGrowthStage),
    AcaciaSapling(SaplingGrowthStage),
    DarkOakSapling(SaplingGrowthStage),
    Bedrock,
    Water(FluidLevel),
    Lava(FluidLevel),
    Sand,
    RedSand,
    Gravel,
    GoldOre,
    DeepslateGoldOre,
    IronOre,
    DeepslateIronOre,
    CoalOre,
    DeepslateCoalOre,
    NetherGoldOre,
    OakLog(Axis),
    SpruceLog(Axis),
    BirchLog(Axis),
    JungleLog(Axis),
    AcaciaLog(Axis),
    DarkOakLog(Axis),
    StrippedSpruceLog(Axis),
    StrippedBirchLog(Axis),
    StrippedJungleLog(Axis),
    StrippedAcaciaLog(Axis),
    StrippedDarkOakLog(Axis),
    StrippedOakLog(Axis),
    OakWood(Axis),
    SpruceWood(Axis),
    BirchWood(Axis),
    JungleWood(Axis),
    AcaciaWood(Axis),
    DarkOakWood(Axis),
    StrippedOakWood(Axis),
    StrippedSpruceWood(Axis),
    StrippedBirchWood(Axis),
    StrippedJungleWood(Axis),
    StrippedAcaciaWood(Axis),
    StrippedDarkOakWood(Axis),
    OakLeaves(LeafDistance, LeafPersistence),
    SpruceLeaves(LeafDistance, LeafPersistence),
    BirchLeaves(LeafDistance, LeafPersistence),
    JungleLeaves(LeafDistance, LeafPersistence),
    AcaciaLeaves(LeafDistance, LeafPersistence),
    DarkOakLeaves(LeafDistance, LeafPersistence),
    AzaleaLeaves(LeafDistance, LeafPersistence),
    FloweringAzaleaLeaves(LeafDistance, LeafPersistence),
    Sponge,
    WetSponge,
    Glass,
    LapisOre,
    DeepslateLapisOre,
    LapisBlock,
    Dispenser(Direction, Triggered),
    Sandstone,
    ChiseledSandstone,
    CutSandstone,
    
}

impl Block {
    pub fn to_value(self) -> Result<crate::VarInt, Error> {
        use crate::VarInt;
        match self {
            Self::Air => VarInt::from_value(0),
            Self::Stone => VarInt::from_value(1),
            Self::Granite => VarInt::from_value(2),
            Self::PolishedGranite => VarInt::from_value(3),
            Self::Diorite => VarInt::from_value(4),
            Self::PolishedDiorite => VarInt::from_value(5),
            Self::Andesite => VarInt::from_value(6),
            Self::PolishedAndesite => VarInt::from_value(7),
            Self::GrassBlock(snowy) => {
                if snowy == Snowy::True {
                    return VarInt::from_value(8);
                }
                VarInt::from_value(9)
            }
            _ => todo!()
        }
    }
    pub fn as_namespaced_id(self) -> String {
        todo!();
    }
}

impl TryFrom<crate::VarInt> for Block {
    type Error = Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        todo!();
        match value {
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

