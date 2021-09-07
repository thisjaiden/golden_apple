use crate::Error;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Represents the destination of a Skulk Vibration particle.
pub enum SkulkVibrationDestination {
    /// This particle is headed to a block at a position.
    BlockPosition(crate::Position),
    /// This particle is headed to an entity with an eid.
    EntityID(crate::VarInt)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
/// Represents the type of a painting.
pub enum PaintingType {
    Kebab = 0,
    Aztec = 1,
    Alban = 2,
    Aztec2 = 3,
    Bomb = 4,
    Plant = 5,
    Wasteland = 6,
    Pool = 7,
    Courbert = 8,
    Sea = 9,
    Sunset = 10,
    Creebet = 11,
    Wanderer = 12,
    Graham = 13,
    Match = 14,
    Bust = 15,
    Stage = 16,
    Void = 17,
    SkullAndRoses = 18,
    Wither = 19,
    Fighters = 20,
    Pointer = 21,
    Pigscene = 22,
    BurningSkull = 23,
    Skeleton = 24,
    DonkeyKong = 25
}

impl TryFrom<crate::VarInt> for PaintingType {
    type Error = Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        use crate::VarInt;
        match value {
            x if x == VarInt::from_value(Self::Kebab as i32)? => Ok(Self::Kebab),
            x if x == VarInt::from_value(Self::Aztec as i32)? => Ok(Self::Aztec),
            x if x == VarInt::from_value(Self::Alban as i32)? => Ok(Self::Alban),
            x if x == VarInt::from_value(Self::Aztec2 as i32)? => Ok(Self::Aztec2),
            x if x == VarInt::from_value(Self::Bomb as i32)? => Ok(Self::Bomb),
            x if x == VarInt::from_value(Self::Plant as i32)? => Ok(Self::Plant),
            x if x == VarInt::from_value(Self::Wasteland as i32)? => Ok(Self::Wasteland),
            x if x == VarInt::from_value(Self::Pool as i32)? => Ok(Self::Pool),
            x if x == VarInt::from_value(Self::Courbert as i32)? => Ok(Self::Courbert),
            x if x == VarInt::from_value(Self::Sea as i32)? => Ok(Self::Sea),
            x if x == VarInt::from_value(Self::Sunset as i32)? => Ok(Self::Sunset),
            x if x == VarInt::from_value(Self::Creebet as i32)? => Ok(Self::Creebet),
            x if x == VarInt::from_value(Self::Wanderer as i32)? => Ok(Self::Wanderer),
            x if x == VarInt::from_value(Self::Graham as i32)? => Ok(Self::Graham),
            x if x == VarInt::from_value(Self::Match as i32)? => Ok(Self::Match),
            x if x == VarInt::from_value(Self::Bust as i32)? => Ok(Self::Bust),
            x if x == VarInt::from_value(Self::Stage as i32)? => Ok(Self::Stage),
            x if x == VarInt::from_value(Self::Void as i32)? => Ok(Self::Void),
            x if x == VarInt::from_value(Self::SkullAndRoses as i32)? => Ok(Self::SkullAndRoses),
            x if x == VarInt::from_value(Self::Wither as i32)? => Ok(Self::Wither),
            x if x == VarInt::from_value(Self::Fighters as i32)? => Ok(Self::Fighters),
            x if x == VarInt::from_value(Self::Pointer as i32)? => Ok(Self::Pointer),
            x if x == VarInt::from_value(Self::Pigscene as i32)? => Ok(Self::Pigscene),
            x if x == VarInt::from_value(Self::BurningSkull as i32)? => Ok(Self::BurningSkull),
            x if x == VarInt::from_value(Self::Skeleton as i32)? => Ok(Self::Skeleton),
            x if x == VarInt::from_value(Self::DonkeyKong as i32)? => Ok(Self::DonkeyKong),
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        match value {
            x if x == Self::South as u8 => Ok(Self::South),
            x if x == Self::West as u8 => Ok(Self::West),
            x if x == Self::North as u8 => Ok(Self::North),
            x if x == Self::East as u8 => Ok(Self::East),
            _ => Err(Error::EnumOutOfBound)
        }
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Orientation {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
/// Represents a non-block thing in the world.
pub enum EntityType {
    AreaEffectCloud = 0,
    ArmorStand = 1,
    Arrow = 2,
    Axolotl = 3,
    Bat = 4,
    Bee = 5,
    Blaze = 6,
    Boat = 7,
    Cat = 8,
    CaveSpider = 9,
    Chicken = 10,
    Cod = 11,
    Cow = 12,
    Creeper = 13,
    Dolphin = 14,
    Donkey = 15,
    DragonFireball = 16,
    Drowned = 17,
    ElderGuardian = 18,
    EndCrystal = 19,
    EnderDragon = 20,
    Enderman = 21,
    Endermite = 22,
    Evoker = 23,
    EvokerFangs = 24,
    ExperienceOrb = 25,
    EyeOfEnder = 26,
    FallingBlock = 27,
    FireworkRocket = 28,
    Fox = 29,
    Ghast = 30,
    Giant = 31,
    GlowItemFrame = 32,
    GlowSquid = 33,
    Goat = 34,
    Guardian = 35,
    Hoglin = 36,
    Horse = 37,
    Husk = 38,
    Illusioner = 39,
    IronGolem = 40,
    Item = 41,
    ItemFrame = 42,
    Fireball = 43,
    LeashKnot = 44,
    LightningBolt = 45,
    Llama = 46,
    LlamaSpit = 47,
    MagmaCube = 48,
    Marker = 49,
    Minecart = 50,
    MinecartChest = 51,
    MinecartCommandBlock = 52,
    MinecartFurnace = 53,
    MinecartHopper = 54,
    MinecartSpawner = 55,
    MinecartTNT = 56,
    Mule = 57,
    Mooshroom = 58,
    Ocelot = 59,
    Painting = 60,
    Panda = 61,
    Parrot = 62,
    Phantom = 63,
    Pig = 64,
    Piglin = 65,
    PiglinBrute = 66,
    Pillager = 67,
    PolarBear = 68,
    PrimedTNT = 69,
    Pufferfish = 70,
    Rabbit = 71,
    Ravager = 72,
    Salmon = 73,
    Sheep = 74,
    Shulker = 75,
    ShulkerBullet = 76,
    Silverfish = 77,
    Skeleton = 78,
    SkeletonHorse = 79,
    Slime = 80,
    SmallFireball = 81,
    SnowGolem = 82,
    Snowball = 83,
    SpectralArrow = 84,
    Spider = 85,
    Squid = 86,
    Stray = 87,
    Strider = 88,
    ThrownEgg = 89,
    ThrownEnderPearl = 90,
    ThrownExperienceBottle = 91,
    ThrownPotion = 92,
    ThrownTrident = 93,
    TraderLlama = 94,
    TropicalFish = 95,
    Turtle = 96,
    Vex = 97,
    Villager = 98,
    Vindicator = 99,
    WanderingTrader = 100,
    Witch = 101,
    Wither = 102,
    WitherSkeleton = 103,
    WitherSkull = 104,
    Wolf = 105,
    Zoglin = 106,
    Zombie = 107,
    ZombieHorse = 108,
    ZombieVillager = 109,
    ZombifiedPiglin = 110,
    Player = 111,
    FishingHook = 112
}

impl TryFrom<crate::VarInt> for EntityType {
    type Error = Error;
    fn try_from(value: crate::VarInt) -> Result<Self, Self::Error> {
        use crate::VarInt;
        match value {
            x if x == VarInt::from_value(Self::AreaEffectCloud as i32)? => Ok(Self::AreaEffectCloud),
            x if x == VarInt::from_value(Self::ArmorStand as i32)? => Ok(Self::ArmorStand),
            x if x == VarInt::from_value(Self::Arrow as i32)? => Ok(Self::Arrow),
            x if x == VarInt::from_value(Self::Axolotl as i32)? => Ok(Self::Axolotl),
            x if x == VarInt::from_value(Self::Bat as i32)? => Ok(Self::Bat),
            x if x == VarInt::from_value(Self::Bee as i32)? => Ok(Self::Bee),
            x if x == VarInt::from_value(Self::Blaze as i32)? => Ok(Self::Blaze),
            x if x == VarInt::from_value(Self::Boat as i32)? => Ok(Self::Boat),
            x if x == VarInt::from_value(Self::Cat as i32)? => Ok(Self::Cat),
            x if x == VarInt::from_value(Self::CaveSpider as i32)? => Ok(Self::CaveSpider),
            x if x == VarInt::from_value(Self::Chicken as i32)? => Ok(Self::Chicken),
            x if x == VarInt::from_value(Self::Cod as i32)? => Ok(Self::Cod),
            x if x == VarInt::from_value(Self::Cow as i32)? => Ok(Self::Cow),
            x if x == VarInt::from_value(Self::Creeper as i32)? => Ok(Self::Creeper),
            x if x == VarInt::from_value(Self::Dolphin as i32)? => Ok(Self::Dolphin),
            x if x == VarInt::from_value(Self::Donkey as i32)? => Ok(Self::Donkey),
            x if x == VarInt::from_value(Self::DragonFireball as i32)? => Ok(Self::DragonFireball),
            x if x == VarInt::from_value(Self::Drowned as i32)? => Ok(Self::Drowned),
            x if x == VarInt::from_value(Self::ElderGuardian as i32)? => Ok(Self::ElderGuardian),
            x if x == VarInt::from_value(Self::EndCrystal as i32)? => Ok(Self::EndCrystal),
            x if x == VarInt::from_value(Self::EnderDragon as i32)? => Ok(Self::EnderDragon),
            x if x == VarInt::from_value(Self::Enderman as i32)? => Ok(Self::Enderman),
            x if x == VarInt::from_value(Self::Endermite as i32)? => Ok(Self::Endermite),
            x if x == VarInt::from_value(Self::Evoker as i32)? => Ok(Self::Evoker),
            x if x == VarInt::from_value(Self::EvokerFangs as i32)? => Ok(Self::EvokerFangs),
            x if x == VarInt::from_value(Self::ExperienceOrb as i32)? => Ok(Self::ExperienceOrb),
            x if x == VarInt::from_value(Self::EyeOfEnder as i32)? => Ok(Self::EyeOfEnder),
            x if x == VarInt::from_value(Self::FallingBlock as i32)? => Ok(Self::FallingBlock),
            x if x == VarInt::from_value(Self::FireworkRocket as i32)? => Ok(Self::FireworkRocket),
            x if x == VarInt::from_value(Self::Fox as i32)? => Ok(Self::Fox),
            x if x == VarInt::from_value(Self::Ghast as i32)? => Ok(Self::Ghast),
            x if x == VarInt::from_value(Self::Giant as i32)? => Ok(Self::Giant),
            x if x == VarInt::from_value(Self::GlowItemFrame as i32)? => Ok(Self::GlowItemFrame),
            x if x == VarInt::from_value(Self::GlowSquid as i32)? => Ok(Self::GlowSquid),
            x if x == VarInt::from_value(Self::Goat as i32)? => Ok(Self::Goat),
            x if x == VarInt::from_value(Self::Guardian as i32)? => Ok(Self::Guardian),
            x if x == VarInt::from_value(Self::Hoglin as i32)? => Ok(Self::Hoglin),
            x if x == VarInt::from_value(Self::Horse as i32)? => Ok(Self::Horse),
            x if x == VarInt::from_value(Self::Husk as i32)? => Ok(Self::Husk),
            x if x == VarInt::from_value(Self::Illusioner as i32)? => Ok(Self::Illusioner),
            x if x == VarInt::from_value(Self::IronGolem as i32)? => Ok(Self::IronGolem),
            x if x == VarInt::from_value(Self::Item as i32)? => Ok(Self::Item),
            x if x == VarInt::from_value(Self::ItemFrame as i32)? => Ok(Self::ItemFrame),
            x if x == VarInt::from_value(Self::Fireball as i32)? => Ok(Self::Fireball),
            x if x == VarInt::from_value(Self::LeashKnot as i32)? => Ok(Self::LeashKnot),
            x if x == VarInt::from_value(Self::LightningBolt as i32)? => Ok(Self::LightningBolt),
            x if x == VarInt::from_value(Self::Llama as i32)? => Ok(Self::Llama),
            x if x == VarInt::from_value(Self::LlamaSpit as i32)? => Ok(Self::LlamaSpit),
            x if x == VarInt::from_value(Self::MagmaCube as i32)? => Ok(Self::MagmaCube),
            x if x == VarInt::from_value(Self::Marker as i32)? => Ok(Self::Marker),
            x if x == VarInt::from_value(Self::Minecart as i32)? => Ok(Self::Minecart),
            x if x == VarInt::from_value(Self::MinecartChest as i32)? => Ok(Self::MinecartChest),
            x if x == VarInt::from_value(Self::MinecartCommandBlock as i32)? => Ok(Self::MinecartCommandBlock),
            x if x == VarInt::from_value(Self::MinecartFurnace as i32)? => Ok(Self::MinecartFurnace),
            x if x == VarInt::from_value(Self::MinecartHopper as i32)? => Ok(Self::MinecartHopper),
            x if x == VarInt::from_value(Self::MinecartSpawner as i32)? => Ok(Self::MinecartSpawner),
            x if x == VarInt::from_value(Self::MinecartTNT as i32)? => Ok(Self::MinecartTNT),
            x if x == VarInt::from_value(Self::Mule as i32)? => Ok(Self::Mule),
            x if x == VarInt::from_value(Self::Mooshroom as i32)? => Ok(Self::Mooshroom),
            x if x == VarInt::from_value(Self::Ocelot as i32)? => Ok(Self::Ocelot),
            x if x == VarInt::from_value(Self::Painting as i32)? => Ok(Self::Painting),
            x if x == VarInt::from_value(Self::Panda as i32)? => Ok(Self::Panda),
            x if x == VarInt::from_value(Self::Parrot as i32)? => Ok(Self::Parrot),
            x if x == VarInt::from_value(Self::Phantom as i32)? => Ok(Self::Phantom),
            x if x == VarInt::from_value(Self::Pig as i32)? => Ok(Self::Pig),
            x if x == VarInt::from_value(Self::Piglin as i32)? => Ok(Self::Piglin),
            x if x == VarInt::from_value(Self::PiglinBrute as i32)? => Ok(Self::PiglinBrute),
            x if x == VarInt::from_value(Self::Pillager as i32)? => Ok(Self::Pillager),
            x if x == VarInt::from_value(Self::PolarBear as i32)? => Ok(Self::PolarBear),
            x if x == VarInt::from_value(Self::PrimedTNT as i32)? => Ok(Self::PrimedTNT),
            x if x == VarInt::from_value(Self::Pufferfish as i32)? => Ok(Self::Pufferfish),
            x if x == VarInt::from_value(Self::Rabbit as i32)? => Ok(Self::Rabbit),
            x if x == VarInt::from_value(Self::Ravager as i32)? => Ok(Self::Ravager),
            x if x == VarInt::from_value(Self::Salmon as i32)? => Ok(Self::Salmon),
            x if x == VarInt::from_value(Self::Sheep as i32)? => Ok(Self::Sheep),
            x if x == VarInt::from_value(Self::Shulker as i32)? => Ok(Self::Shulker),
            x if x == VarInt::from_value(Self::ShulkerBullet as i32)? => Ok(Self::ShulkerBullet),
            x if x == VarInt::from_value(Self::Silverfish as i32)? => Ok(Self::Silverfish),
            x if x == VarInt::from_value(Self::Skeleton as i32)? => Ok(Self::Skeleton),
            x if x == VarInt::from_value(Self::SkeletonHorse as i32)? => Ok(Self::SkeletonHorse),
            x if x == VarInt::from_value(Self::Slime as i32)? => Ok(Self::Slime),
            x if x == VarInt::from_value(Self::SmallFireball as i32)? => Ok(Self::SmallFireball),
            x if x == VarInt::from_value(Self::SnowGolem as i32)? => Ok(Self::SnowGolem),
            x if x == VarInt::from_value(Self::Snowball as i32)? => Ok(Self::Snowball),
            x if x == VarInt::from_value(Self::SpectralArrow as i32)? => Ok(Self::SpectralArrow),
            x if x == VarInt::from_value(Self::Spider as i32)? => Ok(Self::Spider),
            x if x == VarInt::from_value(Self::Squid as i32)? => Ok(Self::Squid),
            x if x == VarInt::from_value(Self::Stray as i32)? => Ok(Self::Stray),
            x if x == VarInt::from_value(Self::Strider as i32)? => Ok(Self::Strider),
            x if x == VarInt::from_value(Self::ThrownEgg as i32)? => Ok(Self::ThrownEgg),
            x if x == VarInt::from_value(Self::ThrownEnderPearl as i32)? => Ok(Self::ThrownEnderPearl),
            x if x == VarInt::from_value(Self::ThrownExperienceBottle as i32)? => Ok(Self::ThrownExperienceBottle),
            x if x == VarInt::from_value(Self::ThrownPotion as i32)? => Ok(Self::ThrownPotion),
            x if x == VarInt::from_value(Self::ThrownTrident as i32)? => Ok(Self::ThrownTrident),
            x if x == VarInt::from_value(Self::TraderLlama as i32)? => Ok(Self::TraderLlama),
            x if x == VarInt::from_value(Self::TropicalFish as i32)? => Ok(Self::TropicalFish),
            x if x == VarInt::from_value(Self::Turtle as i32)? => Ok(Self::Turtle),
            x if x == VarInt::from_value(Self::Vex as i32)? => Ok(Self::Vex),
            x if x == VarInt::from_value(Self::Villager as i32)? => Ok(Self::Villager),
            x if x == VarInt::from_value(Self::Vindicator as i32)? => Ok(Self::Vindicator),
            x if x == VarInt::from_value(Self::WanderingTrader as i32)? => Ok(Self::WanderingTrader),
            x if x == VarInt::from_value(Self::Witch as i32)? => Ok(Self::Witch),
            x if x == VarInt::from_value(Self::Wither as i32)? => Ok(Self::Wither),
            x if x == VarInt::from_value(Self::WitherSkeleton as i32)? => Ok(Self::WitherSkeleton),
            x if x == VarInt::from_value(Self::WitherSkull as i32)? => Ok(Self::WitherSkull),
            x if x == VarInt::from_value(Self::Wolf as i32)? => Ok(Self::Wolf),
            x if x == VarInt::from_value(Self::Zoglin as i32)? => Ok(Self::Zoglin),
            x if x == VarInt::from_value(Self::Zombie as i32)? => Ok(Self::Zombie),
            x if x == VarInt::from_value(Self::ZombieHorse as i32)? => Ok(Self::ZombieHorse),
            x if x == VarInt::from_value(Self::ZombieVillager as i32)? => Ok(Self::ZombieVillager),
            x if x == VarInt::from_value(Self::ZombifiedPiglin as i32)? => Ok(Self::ZombifiedPiglin),
            x if x == VarInt::from_value(Self::Player as i32)? => Ok(Self::Player),
            x if x == VarInt::from_value(Self::FishingHook as i32)? => Ok(Self::FishingHook),
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        match value {
            x if x == ChatSettings::Full as u8 => Ok(ChatSettings::Full),
            x if x == ChatSettings::System as u8 => Ok(ChatSettings::System),
            x if x == ChatSettings::None as u8 => Ok(ChatSettings::None),
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        match value {
            x if x == MessageType::Chat as u8 => Ok(MessageType::Chat),
            x if x == MessageType::System as u8 => Ok(MessageType::System),
            x if x == MessageType::GameInfo as u8 => Ok(MessageType::GameInfo),
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        match value {
            x if x == crate::VarInt::from_value(NextState::Login as i32)? => Ok(NextState::Login),
            x if x == crate::VarInt::from_value(NextState::Status as i32)? => Ok(NextState::Status),
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        match value {
            x if x == ProtocolState::Handshake as u8 => Ok(ProtocolState::Handshake),
            x if x == ProtocolState::Status as u8 => Ok(ProtocolState::Status),
            x if x == ProtocolState::Login as u8 => Ok(ProtocolState::Login),
            x if x == ProtocolState::Play as u8 => Ok(ProtocolState::Play),
             _ => Err(Error::EnumOutOfBound)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
/// Indicates which type of particle is being refrenced.
pub enum ParticleType {
    AmbientEntityEffect = 0,
    /// Angry villager particle.
    AngryVilalger = 1,
    /// Barrier block particle.
    Barrier = 2,
    /// Light block particle.
    Light = 3,
    Block = 4,
    Bubble = 5,
    Cloud = 6,
    /// Critical strike particle.
    Crit = 7,
    DamageIndicator = 8,
    DragonBreath = 9,
    DrippingLava = 10,
    FallingLava = 11,
    LandingLava = 12,
    DrippingWater = 13,
    FallingWater = 14,
    Dust = 15,
    DustColorTransition = 16,
    Effect = 17,
    ElderGuardian = 18,
    EnchantedHit = 19,
    Enchant = 20,
    EndRod = 21,
    EntityEffect = 22,
    ExplosionEmitter = 23,
    Explosion = 24,
    FallingDust = 25,
    Firework = 26,
    Fishing = 27,
    Flame = 28,
    SoulFireFlame = 29,
    Soul = 30,
    Flash = 31,
    HappyVillager = 32,
    Composter = 33,
    Heart = 34,
    InstantEffect = 35,
    Item = 36,
    Vibration = 37,
    ItemSlime = 38,
    ItemSnowball = 39,
    LargeSmoke = 40,
    Lava = 41,
    Mycelium = 42,
    Note = 43,
    Poof = 44,
    Portal = 45,
    Rain = 46,
    Smoke = 47,
    Sneeze = 48,
    Spit = 49,
    SquidInk = 50,
    SweepAttack = 51,
    TotemOfUndying = 52,
    Underwater = 53,
    Splash = 54,
    Witch = 55,
    BubblePop = 56,
    CurrentDown = 57,
    BubbleColumnUp = 58,
    Nautilus = 59,
    Dolphin = 60,
    CampfireCosySmoke = 61,
    CampfireSignalSmoke = 62,
    DrippingHoney = 63,
    FallingHoney = 64,
    LandingHoney = 65,
    FallingNectar = 66,
    FallingSporeBlossom = 67,
    Ash = 68,
    CrimsonSpore = 69,
    WarpedSpore = 70,
    SporeBlossomAir = 71,
    DrippingObsidianTear = 72,
    FallingObsidianTear = 73,
    LandingObsidianTear = 74,
    ReversePortal = 75,
    WhiteAsh = 76,
    SmallFlame = 77,
    Snowflake = 78,
    DrippingDripstoneLava = 79,
    FallingDripstoneLava = 80,
    DrippingDripstoneWater = 81,
    FallingDripstoneWater = 82,
    GlowSquidInk = 83,
    Glow = 84,
    WaxOn = 85,
    WaxOff = 86,
    ElectricSpark = 87,
    Scrape = 88
}
    
impl TryFrom<u8> for ParticleType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::AmbientEntityEffect as u8 => Ok(Self::AmbientEntityEffect),
            x if x == Self::AngryVilalger as u8 => Ok(Self::AngryVilalger),
            x if x == Self::Ash as u8 => Ok(Self::Ash),
            x if x == Self::Barrier as u8 => Ok(Self::Barrier),
            x if x == Self::Block as u8 => Ok(Self::Block),
            x if x == Self::Bubble as u8 => Ok(Self::Bubble),
            x if x == Self::BubbleColumnUp as u8 => Ok(Self::BubbleColumnUp),
            x if x == Self::BubblePop as u8 => Ok(Self::BubblePop),
            x if x == Self::CampfireCosySmoke as u8 => Ok(Self::CampfireCosySmoke),
            x if x == Self::CampfireSignalSmoke as u8 => Ok(Self::CampfireSignalSmoke),
            x if x == Self::Cloud as u8 => Ok(Self::Cloud),
            x if x == Self::Composter as u8 => Ok(Self::Composter),
            x if x == Self::CrimsonSpore as u8 => Ok(Self::CrimsonSpore),
            x if x == Self::Crit as u8 => Ok(Self::Crit),
            x if x == Self::CurrentDown as u8 => Ok(Self::CurrentDown),
            x if x == Self::DamageIndicator as u8 => Ok(Self::DamageIndicator),
            x if x == Self::Dolphin as u8 => Ok(Self::Dolphin),
            x if x == Self::DragonBreath as u8 => Ok(Self::DragonBreath),
            x if x == Self::DrippingDripstoneLava as u8 => Ok(Self::DrippingDripstoneLava),
            x if x == Self::DrippingDripstoneWater as u8 => Ok(Self::DrippingDripstoneWater),
            x if x == Self::DrippingHoney as u8 => Ok(Self::DrippingHoney),
            x if x == Self::DrippingLava as u8 => Ok(Self::DrippingLava),
            x if x == Self::DrippingObsidianTear as u8 => Ok(Self::DrippingObsidianTear),
            x if x == Self::DrippingWater as u8 => Ok(Self::DrippingWater),
            x if x == Self::Dust as u8 => Ok(Self::Dust),
            x if x == Self::DustColorTransition as u8 => Ok(Self::DustColorTransition),
            x if x == Self::Effect as u8 => Ok(Self::Effect),
            x if x == Self::ElderGuardian as u8 => Ok(Self::ElderGuardian),
            x if x == Self::ElectricSpark as u8 => Ok(Self::ElectricSpark),
            x if x == Self::Enchant as u8 => Ok(Self::Enchant),
            x if x == Self::EnchantedHit as u8 => Ok(Self::EnchantedHit),
            x if x == Self::EndRod as u8 => Ok(Self::EndRod),
            x if x == Self::EntityEffect as u8 => Ok(Self::EntityEffect),
            x if x == Self::Explosion as u8 => Ok(Self::Explosion),
            x if x == Self::ExplosionEmitter as u8 => Ok(Self::ExplosionEmitter),
            x if x == Self::FallingDripstoneLava as u8 => Ok(Self::FallingDripstoneLava),
            x if x == Self::FallingDripstoneWater as u8 => Ok(Self::FallingDripstoneWater),
            x if x == Self::FallingDust as u8 => Ok(Self::FallingDust),
            x if x == Self::FallingHoney as u8 => Ok(Self::FallingHoney),
            x if x == Self::FallingLava as u8 => Ok(Self::FallingLava),
            x if x == Self::FallingNectar as u8 => Ok(Self::FallingNectar),
            x if x == Self::FallingObsidianTear as u8 => Ok(Self::FallingObsidianTear),
            x if x == Self::FallingSporeBlossom as u8 => Ok(Self::FallingSporeBlossom),
            x if x == Self::FallingWater as u8 => Ok(Self::FallingWater),
            x if x == Self::Firework as u8 => Ok(Self::Firework),
            x if x == Self::Fishing as u8 => Ok(Self::Fishing),
            x if x == Self::Flame as u8 => Ok(Self::Flame),
            x if x == Self::Flash as u8 => Ok(Self::Flash),
            x if x == Self::Glow as u8 => Ok(Self::Glow),
            x if x == Self::GlowSquidInk as u8 => Ok(Self::GlowSquidInk),
            x if x == Self::HappyVillager as u8 => Ok(Self::HappyVillager),
            x if x == Self::Heart as u8 => Ok(Self::Heart),
            x if x == Self::InstantEffect as u8 => Ok(Self::InstantEffect),
            x if x == Self::Item as u8 => Ok(Self::Item),
            x if x == Self::ItemSlime as u8 => Ok(Self::ItemSlime),
            x if x == Self::ItemSnowball as u8 => Ok(Self::ItemSnowball),
            x if x == Self::LandingHoney as u8 => Ok(Self::LandingHoney),
            x if x == Self::LandingLava as u8 => Ok(Self::LandingLava),
            x if x == Self::LandingObsidianTear as u8 => Ok(Self::LandingObsidianTear),
            x if x == Self::LargeSmoke as u8 => Ok(Self::LargeSmoke),
            x if x == Self::Lava as u8 => Ok(Self::Lava),
            x if x == Self::Light as u8 => Ok(Self::Light),
            x if x == Self::Mycelium as u8 => Ok(Self::Mycelium),
            x if x == Self::Nautilus as u8 => Ok(Self::Nautilus),
            x if x == Self::Note as u8 => Ok(Self::Note),
            x if x == Self::Poof as u8 => Ok(Self::Poof),
            x if x == Self::Portal as u8 => Ok(Self::Portal),
            x if x == Self::Rain as u8 => Ok(Self::Rain),
            x if x == Self::ReversePortal as u8 => Ok(Self::ReversePortal),
            x if x == Self::Scrape as u8 => Ok(Self::Scrape),
            x if x == Self::SmallFlame as u8 => Ok(Self::SmallFlame),
            x if x == Self::Smoke as u8 => Ok(Self::Smoke),
            x if x == Self::Sneeze as u8 => Ok(Self::Sneeze),
            x if x == Self::Snowflake as u8 => Ok(Self::Snowflake),
            x if x == Self::Soul as u8 => Ok(Self::Soul),
            x if x == Self::SoulFireFlame as u8 => Ok(Self::SoulFireFlame),
            x if x == Self::Spit as u8 => Ok(Self::Spit),
            x if x == Self::Splash as u8 => Ok(Self::Splash),
            x if x == Self::SporeBlossomAir as u8 => Ok(Self::SporeBlossomAir),
            x if x == Self::SquidInk as u8 => Ok(Self::SquidInk),
            x if x == Self::SweepAttack as u8 => Ok(Self::SweepAttack),
            x if x == Self::TotemOfUndying as u8 => Ok(Self::TotemOfUndying),
            x if x == Self::Underwater as u8 => Ok(Self::Underwater),
            x if x == Self::Vibration as u8 => Ok(Self::Vibration),
            x if x == Self::WarpedSpore as u8 => Ok(Self::WarpedSpore),
            x if x == Self::WaxOff as u8 => Ok(Self::WaxOff),
            x if x == Self::WaxOn as u8 => Ok(Self::WaxOn),
            x if x == Self::WhiteAsh as u8 => Ok(Self::WhiteAsh),
            x if x == Self::Witch as u8 => Ok(Self::Witch),
            _ => Err(Error::EnumOutOfBound)
        }
    }
}

impl ParticleType {
    /// Returns the Identifier for this ParticleType.
    pub fn to_identifier(self) -> Result<super::Identifier, super::Error> {
        use super::Identifier;
        match self {
            Self::AmbientEntityEffect => return Identifier::from_string(String::from("ambient_entity_effect")),
            Self::AngryVilalger => return Identifier::from_string(String::from("angry_villager")),
            Self::Ash => return Identifier::from_string(String::from("ash")),
            Self::Barrier => return Identifier::from_string(String::from("barrier")),
            Self::Block => return Identifier::from_string(String::from("block")),
            Self::Bubble => return Identifier::from_string(String::from("bubble")),
            Self::BubbleColumnUp => return Identifier::from_string(String::from("bubble_column_up")),
            Self::BubblePop => return Identifier::from_string(String::from("bubble_pop")),
            Self::CampfireCosySmoke => return Identifier::from_string(String::from("campfire_cosy_smoke")),
            Self::CampfireSignalSmoke => return Identifier::from_string(String::from("campfire_signal_smoke")),
            Self::Cloud => return Identifier::from_string(String::from("cloud")),
            Self::Composter => return Identifier::from_string(String::from("composter")),
            Self::CrimsonSpore => return Identifier::from_string(String::from("crimson_spore")),
            Self::Crit => return Identifier::from_string(String::from("crit")),
            Self::CurrentDown => return Identifier::from_string(String::from("current_down")),
            Self::DamageIndicator => return Identifier::from_string(String::from("damage_indicator")),
            Self::Dolphin => return Identifier::from_string(String::from("dolphin")),
            Self::DragonBreath => return Identifier::from_string(String::from("dragon_breath")),
            Self::DrippingDripstoneLava => return Identifier::from_string(String::from("dripping_dripstone_lava")),
            Self::DrippingDripstoneWater => return Identifier::from_string(String::from("dripping_dripstone_water")),
            Self::DrippingHoney => return Identifier::from_string(String::from("dripping_honey")),
            Self::DrippingLava => return Identifier::from_string(String::from("dripping_lava")),
            Self::DrippingObsidianTear => return Identifier::from_string(String::from("dripping_obsidian_tear")),
            Self::DrippingWater => return Identifier::from_string(String::from("dripping_water")),
            Self::Dust => return Identifier::from_string(String::from("dust")),
            Self::DustColorTransition => return Identifier::from_string(String::from("dust_color_transition")),
            Self::Effect => return Identifier::from_string(String::from("effect")),
            Self::ElderGuardian => return Identifier::from_string(String::from("elder_guardian")),
            Self::ElectricSpark => return Identifier::from_string(String::from("electric_spark")),
            Self::Enchant => return Identifier::from_string(String::from("enchant")),
            Self::EnchantedHit => return Identifier::from_string(String::from("enchanted_hit")),
            Self::EndRod => return Identifier::from_string(String::from("end_rod")),
            Self::EntityEffect => return Identifier::from_string(String::from("entity_effect")),
            Self::Explosion => return Identifier::from_string(String::from("explosion")),
            Self::ExplosionEmitter => return Identifier::from_string(String::from("explosion_emitter")),
            Self::FallingDripstoneLava => return Identifier::from_string(String::from("falling_dripstone_lava")),
            Self::FallingDripstoneWater => return Identifier::from_string(String::from("falling_dripstone_water")),
            Self::FallingDust => return Identifier::from_string(String::from("falling_dust")),
            Self::FallingHoney => return Identifier::from_string(String::from("falling_honey")),
            Self::FallingLava => return Identifier::from_string(String::from("falling_lava")),
            Self::FallingNectar => return Identifier::from_string(String::from("falling_nectar")),
            Self::FallingObsidianTear => return Identifier::from_string(String::from("falling_obsidian_tear")),
            Self::FallingSporeBlossom => return Identifier::from_string(String::from("falling_spore_blossom")),
            Self::FallingWater => return Identifier::from_string(String::from("falling_water")),
            Self::Firework => return Identifier::from_string(String::from("firework")),
            Self::Fishing => return Identifier::from_string(String::from("fishing")),
            Self::Flame => return Identifier::from_string(String::from("flame")),
            Self::Flash => return Identifier::from_string(String::from("flash")),
            Self::Glow => return Identifier::from_string(String::from("glow")),
            Self::GlowSquidInk => return Identifier::from_string(String::from("glow_squid_ink")),
            Self::HappyVillager => return Identifier::from_string(String::from("happy_villager")),
            Self::Heart => return Identifier::from_string(String::from("heart")),
            Self::InstantEffect => return Identifier::from_string(String::from("instant_effect")),
            Self::Item => return Identifier::from_string(String::from("item")),
            Self::ItemSlime => return Identifier::from_string(String::from("item_slime")),
            Self::ItemSnowball => return Identifier::from_string(String::from("item_snowball")),
            Self::LandingHoney => return Identifier::from_string(String::from("landing_honey")),
            Self::LandingLava => return Identifier::from_string(String::from("landing_lava")),
            Self::LandingObsidianTear => return Identifier::from_string(String::from("landing_obsidian_tear")),
            Self::LargeSmoke => return Identifier::from_string(String::from("large_smoke")),
            Self::Lava => return Identifier::from_string(String::from("lava")),
            Self::Light => return Identifier::from_string(String::from("light")),
            Self::Mycelium => return Identifier::from_string(String::from("mycelium")),
            Self::Nautilus => return Identifier::from_string(String::from("nautilus")),
            Self::Note => return Identifier::from_string(String::from("note")),
            Self::Poof => return Identifier::from_string(String::from("poof")),
            Self::Portal => return Identifier::from_string(String::from("portal")),
            Self::Rain => return Identifier::from_string(String::from("rain")),
            Self::ReversePortal => return Identifier::from_string(String::from("reverse_portal")),
            Self::Scrape => return Identifier::from_string(String::from("scrape")),
            Self::SmallFlame => return Identifier::from_string(String::from("small_flame")),
            Self::Smoke => return Identifier::from_string(String::from("smoke")),
            Self::Sneeze => return Identifier::from_string(String::from("sneeze")),
            Self::Snowflake => return Identifier::from_string(String::from("snowflake")),
            Self::Soul => return Identifier::from_string(String::from("soul")),
            Self::SoulFireFlame => return Identifier::from_string(String::from("soul_fire_flame")),
            Self::Spit => return Identifier::from_string(String::from("spit")),
            Self::Splash => return Identifier::from_string(String::from("splash")),
            Self::SporeBlossomAir => return Identifier::from_string(String::from("spore_blossom_air")),
            Self::SquidInk => return Identifier::from_string(String::from("squid_ink")),
            Self::SweepAttack => return Identifier::from_string(String::from("sweep_attack")),
            Self::TotemOfUndying => return Identifier::from_string(String::from("totem_of_undying")),
            Self::Underwater => return Identifier::from_string(String::from("underwater")),
            Self::Vibration => return Identifier::from_string(String::from("vibration")),
            Self::WarpedSpore => return Identifier::from_string(String::from("warped_spore")),
            Self::WaxOff => return Identifier::from_string(String::from("wax_off")),
            Self::WaxOn => return Identifier::from_string(String::from("wax_on")),
            Self::WhiteAsh => return Identifier::from_string(String::from("white_ash")),
            Self::Witch => return Identifier::from_string(String::from("witch")),
        }
    }
}