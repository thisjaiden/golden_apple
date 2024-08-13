use crate::Error;
use std::convert::TryFrom;

// Import of autogenerated files
include!(concat!(env!("OUT_DIR"), "/potion_effects.rs"));
include!(concat!(env!("OUT_DIR"), "/blocks.rs"));
include!(concat!(env!("OUT_DIR"), "/entity_types.rs"));
include!(concat!(env!("OUT_DIR"), "/items.rs"));
include!(concat!(env!("OUT_DIR"), "/particles.rs"));
include!(concat!(env!("OUT_DIR"), "/professions.rs"));
include!(concat!(env!("OUT_DIR"), "/custom_stats.rs"));

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
/// Represents what specific statistic id is being referenced.
pub enum StatisticID {
    /// This statistic references the block registry ids.
    Block(Block),
    /// This statistic references the item registry ids.
    Item(Item),
    /// This statistic references the entity registry ids.
    Entity(EntityType),
    /// This statistic references the custom statistic registry ids.
    Custom(CustomStatistic)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(i32)]
/// Categorizes statstics into smaller groups.
pub enum StatisticCateogry {
    /// How many times this player has mined ___.
    Mined = 0,
    /// How many times this player has crafted ___.
    Crafted = 1,
    /// How many times this player has used ___.
    Used = 2,
    /// How many times this player has broken ___.
    Broken = 3,
    /// How many times this player has picked up ___.
    PickedUp = 4,
    /// How many times this player has dropped ___.
    Dropped = 5,
    /// How many times this player has killed ___.
    Killed = 6,
    /// How many times this player has been killed by ___.
    KilledBy = 7,
    /// Custom statistic.
    Custom = 8
}

impl TryFrom<crate::VarInt> for StatisticCateogry {
    type Error = Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_i32(value.value()).ok_or(Error::EnumOutOfBound);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum DiggingStatus {
    StartDigging = 0,
    CancelDigging = 1,
    FinishDigging = 2
}

impl TryFrom<crate::VarInt> for DiggingStatus {
    type Error = Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        match value.value() {
            x if x == Self::StartDigging as i32 => Ok(Self::StartDigging),
            x if x == Self::CancelDigging as i32 => Ok(Self::CancelDigging),
            x if x == Self::FinishDigging as i32 => Ok(Self::FinishDigging),
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum Animation {
    SwingMainArm = 0,
    TakeDamage = 1,
    LeaveBed = 2,
    SwingOffhand = 3,
    CriticalEffect = 4,
    MagicCriticalEffect = 5
}

impl TryFrom<u8> for Animation {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_u8(value).ok_or(Error::EnumOutOfBound);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Represents the destination of a Skulk Vibration particle.
pub enum SkulkVibrationDestination {
    /// This particle is headed to a block at a position.
    BlockPosition(crate::Position),
    /// This particle is headed to an entity with an eid.
    EntityID(crate::VarInt)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
/// Represents the direction a painting is facing.
pub enum PaintingDirection {
    South = 0,
    West = 1,
    North = 2,
    East = 3
}

impl TryFrom<u8> for PaintingDirection {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_u8(value).ok_or(Error::EnumOutOfBound);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpawnEntityData {
    None,
    HasVelocity(bool),
    MinecartFunctionality(MinecartFunctionality),
    Orientation(Orientation),
    BlockType,
    EntityID(i32)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum MinecartFunctionality {
    Empty = 0,
    Chest = 1,
    Furnace = 2,
    TNT = 3,
    Spawner = 4,
    Hopper = 5,
    CommandBlock = 6
}

impl TryFrom<crate::VarInt> for MinecartFunctionality {
    type Error = Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_i32(value.value()).ok_or(Error::EnumOutOfBound);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Orientation {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
/// Represents the level of chat messages a given client would like to receive.
pub enum ChatSettings {
    /// "The client is willing to accept all chat messages."
    Full = 0,
    /// "The client is willing to accept messages from commands, but does not want general chat
    /// from other players."
    System = 1,
    /// "The client does not want any chat at all. (However, it is still fine with above-hotbar
    /// game notices)"
    None = 2
}

impl TryFrom<u8> for ChatSettings {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_u8(value).ok_or(Error::EnumOutOfBound);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
/// Represents the type of chat message being sent.
pub enum MessageType {
    /// "A player-initiated chat message. Note that the Notchian server does not include
    /// message-related commands here (/me and /tell); those go in System."
    Chat = 0,
    /// "Feedback from running a command, such as 'Your game mode has been updated to creative.'"
    System = 1,
    /// "Game state information that is displayed above the hot bar, such as 'You may not rest
    /// now, the bed is too far away'."
    GameInfo = 2
}
    
impl TryFrom<u8> for MessageType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_u8(value).ok_or(Error::EnumOutOfBound);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(i32)]
/// Indicates what state to switch to to choose the right section of the protocol.
pub enum NextState {
    /// Switch to the Status state. (used for the server list)
    Status = 1,
    /// Switch to the Login state.
    Login = 2
}

impl TryFrom<crate::VarInt> for NextState {
    type Error = Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_i32(value.value()).ok_or(Error::EnumOutOfBound);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
/// Indicates the current section of the network protocol to use.
pub enum ProtocolState {
    /// The Handshake state is used to confirm connection and choose the next state.
    Handshake = 0,
    /// The Status state is for getting information for use in the server list.
    Status = 1,
    /// The Login state is for encrypting, compressing, and preparing for the Play state.
    Login = 2,
    /// The Play state is for standard gameplay.
    Play = 3
}
impl TryFrom<u8> for ProtocolState {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_u8(value).ok_or(Error::EnumOutOfBound);
    }
}
