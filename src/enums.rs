use crate::Error;
use std::convert::TryFrom;

mod block;
pub use block::*;

mod item;
pub use item::Item;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
#[repr(i32)]
/// Represents a custom statistic.
pub enum CustomStatistic {
    /// Amount of times this player has left the game.
    LeaveGame = 0,
    /// Amount of minutes this player has played the game.
    PlayOneMinute = 1,
    /// Amount of ticks since this player has died.
    TimeSinceDeath = 2,
    /// Amount of ticks since this player has sept.
    TimeSinceRest = 3,
    /// Amount of ticks this player has spent sneaking.
    SneakTime = 4,
    /// Hundreths of blocks this player has walked.
    WalkOneCm = 5,
    /// Hundreths of blocks this player has crouch walked.
    CrouchOneCm = 6,
    /// Hundreths of blocks this player has sprinted.
    SprintOneCm = 7,
    /// Hundreths of blocks this player has walked over water using frost walker boots.
    WalkOnWaterOneCm = 8,
    /// Hundreths of blocks this player has fallen.
    FallOneCm = 9,
    /// Hundreths of blocks this player has climbed.
    ClimbOneCm = 10,
    /// Hundreths of blocks this player has flown.
    FlyOneCm = 11,
    /// Hundreths of blocks this player has walked in water.
    WalkUnderWaterOneCm = 12,
    /// Hundreths of blocks this player has travelled in a minecart.
    MinecartOneCm = 13,
    /// Hundreths of blocks this player has rowwed in a boat.
    BoatOneCm = 14,
    /// Hundreths of blocks this player has ridden on a pig.
    PigOneCm = 15,
    /// Hundreths of blocks this player has ridden on a horse.
    HorseOneCm = 16,
    /// Hundreths of blocks this player has levitated.
    AviateOneCm = 17,
    /// Hundreths of blocks this player has swam.
    SwimOneCm = 18,
    /// Hundreths of blocks this player has ridden on a strider.
    StriderOneCm = 19,
    /// Number of times this player has jumped.
    Jump = 20,
    /// Number of times this player has dropped an item.
    Drop = 21,
    /// Tenths of hitpoints done by this player.
    DamageDealt = 22,
    /// Tenths of hitpoints done by this player that were absorbed.
    DamageDealtAbsorbed = 23,
    /// Tenths of hitpoints done by this player that were resisted.
    DamageDealtResisted = 24,
    /// Tenths of hitpoints of damage taken by this player.
    DamageTaken = 25,
    /// Tenths of hitpoints of damage this player has blocked with a shield.
    DamageBlockedByShield = 26,
    /// Tenths of hitpoints of damage this player has absorbed.
    DamageAbsorbed = 27,
    /// Tenths of hitpoints of damage this player has resisted.
    DamageResisted = 28,
    /// Number of times this player has died.
    Deaths = 29,
    /// Number of mobs this player has killed.
    MobKills = 30,
    /// Number of animals this player has bred.
    AnimalsBred = 31,
    /// Number of players this player has killed.
    PlayerKills = 32,
    /// Number of fish this player has caught.
    FishCaught = 33,
    /// Number of times this player has opened the villager GUI.
    TalkedToVillager = 34,
    /// Number of times this player has traded with a villager.
    TradedWithVillager = 35,
    /// Number of times this player has eaten a piece of cake.
    EatCakeSlice = 36,
    /// Number of times this player has filled a cauldron.
    FillCauldron = 37,
    /// Number of times this player has interacted with a cauldron.
    UseCauldron = 38,
    /// Number of times this player has removed dye from leather armor.
    CleanArmor = 39,
    /// Number of times this player has removed patterns from banners.
    CleanBanner = 40,
    /// Number of times this player has removed dye from shulker boxes.
    CleanShulkerBox = 41,
    /// Number of times this player has opened the brewing stand GUI.
    InteractWithBrewingstand = 42,
    /// Number of times this player has opened the beacon GUI.
    InteractWithBeacon = 43,
    /// Number of times this player has opened the dropper GUI.
    InspectDropper = 44,
    /// Number of times this player has opened the hopper GUI.
    InspectHopper = 45,
    /// Number of times this player has opened the dispenser GUI.
    InspectDispenser = 46,
    /// Number of times this player has played a noteblock.
    PlayNoteblock = 47,
    /// Number of times this player has changed the pitch of a noteblock.
    TuneNoteblock = 48,
    /// Number of times this player has planted something in a flowerpot.
    PotFlower = 49,
    /// Number of times this player has activated a trapped chest.
    TriggerTrappedChest = 50,
    /// Number of times this player has opened the enderchest GUI.
    OpenEnderchest = 51,
    /// Number of times this player has enchanted an item.
    EnchantItem = 52,
    /// Number of times this player has played a music disk in a jukebox.
    PlayRecord = 53,
    /// Number of times this player has opened the furnace GUI.
    InteractWithFurnace = 54,
    /// Number of times this player has opened the crafting table GUI.
    InteractWithCraftingTable = 55,
    /// Number of times this player has opened the chest GUI.
    OpenChest = 56,
    /// Number of times this player has slept in a bed.
    SleepInBed = 57,
    /// Number of times this player has opened the shulker box GUI.
    OpenShulkerBox = 58,
    /// Number of times this player has opened the barrel GUI.
    OpenBarrel = 59,
    /// Number of times this player has opened the blast furnace GUI.
    InteractWithBlastFurnace = 60,
    /// Number of times this player has opened the smoker GUI.
    InteractWithSmoker = 61,
    /// Number of times this player has placed or removed a book on a lectern.
    InteractWithLectern = 62,
    /// Number of times this player has placed or removed an item on a campfire.
    InteractWithCampfire = 63,
    /// Number of times this player has opened the cartography table GUI.
    InteractWithCartographyTable = 64,
    /// Number of times this player has opened the loom GUI.
    InteractWithLoom = 65,
    /// Number of times this player has opened the stonecutter GUI.
    InteractWithStonecutter = 66,
    /// Number of times this player has rung a bell.
    BellRing = 67,
    /// Number of times this player has caused a raid.
    RaidTrigger = 68,
    /// Number of times this player has helped defeat a raid.
    RaidWin = 69,
    /// Number of times this player has opened the anvil GUI.
    InteractWithAnvil = 70,
    /// Number of times this player has opened the grindstone GUI.
    InteractWithGrindstone = 71,
    /// Number of times this player has successfully hit a target block.
    TargetHit = 72,
    /// Number of times this player has opened th smithing table GUI.
    InteractWithSmithingTable = 73
}

impl TryFrom<crate::VarInt> for CustomStatistic {
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
        return num_traits::FromPrimitive::from_i32(value.value()).ok_or(Error::EnumOutOfBound);
    }
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
        return num_traits::FromPrimitive::from_i32(value.value()).ok_or(Error::EnumOutOfBound);
    }
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, ToPrimitive)]
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
        return num_traits::FromPrimitive::from_u8(value).ok_or(Error::EnumOutOfBound);
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