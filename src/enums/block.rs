use crate::Error;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Snowy {
    True,
    False
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
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
            _ => Err(Error::EnumOutOfBound)
        }
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

