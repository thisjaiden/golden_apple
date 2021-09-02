//! # Overview
//! `golden_apple` is a library for decoding, encoding, and using common types found in Minecraft:
//! Java Edition.
//!
//! # Goals
//! - Provide a generalized format for sharing and using Minecraft's data types
//! - Simplify the decoding and encoding of network data
//!
//! # Usage
//! Proprietary Minecraft types like `VarInt`, `VarLong`, and `Position` are a part of the top
//! level crate. Types that can be fully represented in Rust have encoders/decoders under
//! `golden_apple::generalized`, in case it isn't striaghtforward to do so.

#[derive(Debug)]
/// Represents an error that can occur while using one of the libraries functions.
pub enum Error {
    /// The datastream representing a VarInt exceded the maximum acceptable size.
    VarIntTooLong,
    /// An error occured while using a `Read` type to parse.
    ReaderError(std::io::Error),
    /// An error occured while using a `Write` type to parse.
    WriterError(std::io::Error),
    /// There was not enough data present to parse.
    MissingData,
    /// A boolean had a value other than true or false.
    InvalidBool,
    /// While reading NBT, the stream started with a value other than 0x0a.
    InvalidNBTHeader,
    /// While reading NBT, the stream had an invalid data type ID.
    InvalidNBTType,
    /// While writing NBT, the root tag was not Tag::Compound.
    InvalidRootTag,
    /// The given identifier had more than one `:`, rendering it invalid.
    InvalidIdentifier,
    /// A given ID for an Enum was out of valid bounds for that type.
    EnumOutOfBound,
    /// An error occured parsing JSON data using `serde_json`.
    JsonParsingError(serde_json::Error),
    /// A JSON tag had a weird root structure.
    InvalidJsonRoot,
    /// A UUID consited of characters other than 0-f
    InvalidUUID(std::num::ParseIntError)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        return Error::JsonParsingError(e);
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        return Error::InvalidUUID(e);
    }
}

impl std::error::Error for Error {}

/// Represents a Unique User ID. Used to track players and entities.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UUID {
    /// The value of this UUID
    value: u128
}

impl UUID {
    /// Generates a UUID from a Read type.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<UUID, Error> {
        return Ok(Self::from_bytes(&[read_byte(reader)?; 16])?.0);
    }
    /// Generates a UUID from a byte array. Returns the UUID and amount of bytes needed.
    pub fn from_bytes(data: &[u8]) -> Result<(UUID, usize), Error> {
        if data.len() < 16 {
            return Err(Error::MissingData);
        }
        let mut array = [0;16];
        for i in 0..16 {
            array[i] = data[i];
        }
        return Ok((Self::from_value(u128::from_be_bytes(array))?, 16));
    }
    /// Generates a UUID from a given value.
    pub fn from_value(value: u128) -> Result<UUID, Error> {
        return Ok(UUID { value });
    }
    /// Generates a UUID from a username. This function uses Mojang's API, and may be subject to
    /// rate limiting. Cache your results.
    pub fn from_username(username: String) -> Result<UUID, Error> {
        use reqwest::blocking::get;
        let raw_response = get(format!("https://api.mojang.com/users/profiles/minecraft/{}", username)).unwrap().text().unwrap();
        let json_response: serde_json::Value = serde_json::from_str(&raw_response)?;
        return Self::from_value(u128::from_str_radix(&json_response["id"].as_str().ok_or(Error::InvalidJsonRoot)?, 16)?);
    }
    /// Writes this UUID to a Write type.
    pub fn to_writer<W: std::io::Write>(self, writer: &mut W) -> Result<(), Error> {
        match writer.write_all(&self.value.to_be_bytes()) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
        return Ok(());
    }
    /// Creates a byte array with the data of this UUID in it.
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        return Ok(self.value.to_be_bytes().to_vec());
    }
    /// Gives the underlying value of this UUID.
    pub fn to_value(self) -> Result<u128, Error> {
        return Ok(self.value);
    }
    /// Gives the username associated with this UUID. This function uses Mojang's API, and may be
    /// subject to rate limiting. Cache your results.
    pub fn to_username(self) -> Result<String, Error> {
        use reqwest::blocking::get;
        let mut insertable = format!("{:x}", self.value);
        insertable = insertable.split('x').next_back().unwrap().to_string();
        while insertable.len() < 32 {
            insertable = String::from("0") + &insertable;
        }
        let raw_response = get(format!("https://api.mojang.com/user/profiles/{}/names", insertable)).unwrap().text().unwrap();
        let json_response: serde_json::Value = serde_json::from_str(&raw_response)?;
        if json_response.is_array() {
            let json_response = json_response.as_array().unwrap();
            return Ok(json_response[json_response.len() - 1]["name"].as_str().ok_or(Error::InvalidJsonRoot)?.to_string());
        }
        else {
            return Err(Error::InvalidJsonRoot);
        }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, PartialEq)]
/// Represents a chat message or other form of rich text.
pub struct Chat {
    component: ChatComponent
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[allow(non_snake_case)]
/// Represents one component of a Chat object.
pub struct ChatComponent {
    /// Text to be used.
    pub text: Option<String>,
    /// Translation key to be used.
    pub translate: Option<String>,
    /// Key to use the translated keybind for.
    pub keybind: Option<String>,
    /// Scoreboard to use.
    pub score: Option<ChatScore>,
    /// Selector to use with `score`.
    pub selector: Option<String>,
    /// Declares if the text is bold.
    pub bold: Option<bool>,
    /// Declares if the text is italic.
    pub italic: Option<bool>,
    /// Declares if the text is underlined.
    pub underlined: Option<bool>,
    /// Declares if the text has a strikethrough applied to it.
    pub strikethrough: Option<bool>,
    /// Declares if the text is obfuscated.
    pub obfuscated: Option<bool>,
    /// Declares the color of the text.
    pub color: Option<String>,
    /// Declares text to insert into the client's chat when clicked.
    pub insertion: Option<String>,
    /// Defines an event when this text is clicked.
    pub clickEvent: Option<ClickEvent>,
    /// Defines an event when a client is hovering over this text.
    pub hoverEvent: Option<HoverEvent>,
    /// Declares extra components to add aftr this one.
    pub extra: Option<Vec<ChatComponent>>
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
/// Describes details about a scoreboard.
pub struct ChatScore {
    /// Name of the given scoreboard.
    pub name: String,
    /// Objective of the given scoreboard.
    pub objective: String,
    /// Value to assign to the given scoreboard.
    pub value: Option<String>
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ClickEvent {
    pub action: String,
    pub value: String
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct HoverEvent {
    pub action: String,
    pub value: String
}

impl Chat {
    pub fn from_bytes(data: &[u8]) -> Result<(Chat, usize), Error> {
        let string_data = generalized::string_from_bytes(data)?;
        return Ok((Self::from_string(string_data.0)?, string_data.1));
    }
    pub fn from_reader<R: std::io::Read>(read: &mut R) -> Result<Chat, Error> {
        return Self::from_string(generalized::string_from_reader(read)?);
    }
    pub fn from_string(data: String) -> Result<Chat, Error> {
        let structure: serde_json::Value = serde_json::from_str(&data)?;
        if structure.is_object() {
            return Ok(Chat {
                component: serde_json::from_str(&data)?
            });
        }
        else if structure.is_array() {
            return Ok(Chat {
                component: ChatComponent {
                    text: None,
                    translate: None,
                    keybind: None,
                    score: None,
                    selector: None,
                    bold: None,
                    italic: None,
                    underlined: None,
                    strikethrough: None,
                    obfuscated: None,
                    color: None,
                    insertion: None,
                    clickEvent: None,
                    hoverEvent: None,
                    extra: serde_json::from_str(&data)?
                }
            });
        }
        else if structure.is_string() {
            return Ok(Chat {
                component: ChatComponent {
                    text: serde_json::from_str(&data)?,
                    translate: None,
                    keybind: None,
                    score: None,
                    selector: None,
                    bold: None,
                    italic: None,
                    underlined: None,
                    strikethrough: None,
                    obfuscated: None,
                    color: None,
                    insertion: None,
                    clickEvent: None,
                    hoverEvent: None,
                    extra: None
                }
            });
        }
        else {
            return Err(Error::InvalidJsonRoot);
        }
    }
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        return generalized::string_to_bytes(serde_json::to_string(&self.component)?);
    }
    pub fn to_writer<W: std::io::Write>(self, writer: &mut W) -> Result<(), Error> {
        generalized::string_to_writer(writer, serde_json::to_string(&self.component)?)?;
        return Ok(());
    }
    pub fn to_string(self) -> Result<String, Error> {
        return Ok(serde_json::to_string(&self.component)?);
    }
}


/// Provides tools for reading, writing, and managing the various enums that Minecraft uses.
/// Many of these enums contain descriptions of their respective attributes in quotes. This
/// indicates that the information is taken directly from https://wiki.vg/Protocol_FAQ
pub mod enums {
    use crate::Error;
    use std::convert::TryFrom;

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
    impl ChatSettings {
        /// Creates a byte representation of this enum.
        pub fn to_byte(self) -> u8 {
            return self as u8;
        }
        /// Attempts to create an enum from a byte value.
        pub fn from_byte(byte: u8) -> Result<Self, Error> {
            use std::convert::TryInto;
            match byte.try_into() {
                Ok(enumval) => {
                    return Ok(enumval);
                }
                Err(e) => {
                    return Err(e);
                }
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
    impl MessageType {
        /// Creates a byte representation of this enum.
        pub fn to_byte(self) -> u8 {
            return self as u8;
        }
        /// Attempts to create an enum from a byte value.
        pub fn from_byte(byte: u8) -> Result<Self, Error> {
            use std::convert::TryInto;
            match byte.try_into() {
                Ok(enumval) => {
                    return Ok(enumval);
                }
                Err(e) => {
                    return Err(e);
                }
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
    impl ProtocolState {
        /// Creates a byte representation of this enum.
        pub fn to_byte(self) -> u8 {
            return self as u8;
        }
        /// Attempts to create an enum from a byte value.
        pub fn from_byte(byte: u8) -> Result<Self, Error> {
            use std::convert::TryInto;
            match byte.try_into() {
                Ok(enumval) => {
                    return Ok(enumval);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    /// Indicates which type of particle is being refrenced.
    pub enum ParticleType {
        AmbientEntityEffect = 0,
        AngryVilalger = 1,
        Barrier = 2,
        Light = 3,
        Block = 4,
        Bubble = 5,
        Cloud = 6,
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
        /// Creates a byte representation of this enum.
        pub fn to_byte(self) -> u8 {
            return self as u8;
        }
        /// Attempts to create an enum from a byte value.
        pub fn from_byte(byte: u8) -> Result<Self, Error> {
            use std::convert::TryInto;
            match byte.try_into() {
                Ok(enumval) => {
                    return Ok(enumval);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

/// Provides tools for reading, writing, and managing NBT types.
pub mod nbt {
    use super::{Error, read_byte};
    /// Reads an entire NBT compound from a Read type.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<NamedTag, Error> {
        if read_byte(reader)? != 0x0a {
            return Err(Error::InvalidNBTHeader);
        }
        let root_name = named_tag_name_reader(reader)?;
        let mut elements = vec![];
        loop {
            let next_tag = read_named_tag(reader)?;
            match next_tag.tag {
                Tag::End => {
                    break;
                }
                _ => {
                    elements.push(next_tag);
                }
            }
        }
        return Ok(NamedTag { name: root_name, tag: Tag::Compound(elements) });
    }
    /// Converts an entire NBT compound into an array of bytes. This must be a full NBT compound.
    pub fn to_bytes(root_tag: NamedTag) -> Result<Vec<u8>, Error> {
        let mut final_bytes = vec![];
        // Add start tag
        final_bytes.push(0x0a);
        // Add root tag name
        for byte in root_tag.name.as_bytes() {
            final_bytes.push(*byte);
        }
        // Add root tag components
        if let Tag::Compound(cmptag) = root_tag.tag {
            for tag in cmptag {
                let prefix = tag.tag.clone().tag_prefix();
                final_bytes.push(prefix);
                if prefix == 0 {
                    break;
                }
                let name = tag.name.as_bytes();
                for byte in &(name.len() as u16).to_be_bytes() {
                    final_bytes.push(*byte);
                }
                for byte in name {
                    final_bytes.push(*byte);
                }
                for byte in tag.tag.write_to_bytes()? {
                    final_bytes.push(byte);
                }
            }
        }
        else {
            return Err(Error::InvalidRootTag);
        }
        // Add end tag
        final_bytes.push(0x00);
        return Ok(final_bytes);
    }
    fn named_tag_name_reader<R: std::io::Read>(reader: &mut R) -> Result<String, Error> {
        let string_len = u16::from_be_bytes([read_byte(reader)?; 2]);
        let mut bytes = vec![];
        for _ in 0..string_len {
            bytes.push(read_byte(reader)?);
        }
        // This is required because Mojang uses Java's modified utf-8 which isn't supported here
        unsafe {
            let string = String::from_utf8_unchecked(bytes);
            return Ok(string);
        }
    }
    fn read_named_tag<R: std::io::Read>(reader: &mut R) -> Result<NamedTag, Error> {
        let tag_type = read_byte(reader)?;
        let tag_name;
        if !(tag_type == 0x00) {
            tag_name = named_tag_name_reader(reader)?;
        }
        else {
            tag_name = String::from("N/A");
        }
        let tag_val = read_from_type(reader, tag_type)?;
        return Ok(NamedTag { name: tag_name, tag: tag_val });
    }
    fn read_tag<R: std::io::Read>(reader: &mut R) -> Result<Tag, Error> {
        let tag_type = read_byte(reader)?;
        let tag_val = read_from_type(reader, tag_type)?;
        return Ok(tag_val);
    }
    fn read_from_type<R: std::io::Read>(reader: &mut R, type_id: u8) -> Result<Tag, Error> {
        match type_id {
            0x00 => {
                return Ok(Tag::End);
            }
            0x01 => {
                return Ok(Tag::Byte(i8::from_be_bytes([read_byte(reader)?])));
            }
            0x02 => {
                return Ok(Tag::Short(i16::from_be_bytes([read_byte(reader)?; 2])));
            }
            0x03 => {
                return Ok(Tag::Int(i32::from_be_bytes([read_byte(reader)?; 4])));
            }
            0x04 => {
                return Ok(Tag::Long(i64::from_be_bytes([read_byte(reader)?; 8])));
            }
            0x05 => {
                return Ok(Tag::Float(f32::from_be_bytes([read_byte(reader)?; 4])));
            }
            0x06 => {
                return Ok(Tag::Double(f64::from_be_bytes([read_byte(reader)?; 8])));
            }
            0x07 => {
                let array_len = i32::from_be_bytes([read_byte(reader)?; 4]);
                let mut array = vec![];
                for _ in 0..array_len {
                    array.push(i8::from_be_bytes([read_byte(reader)?]));
                }
                return Ok(Tag::ByteArray(array));
            }
            0x08 => {
                return Ok(Tag::String(named_tag_name_reader(reader)?));
            }
            0x09 => {
                let _list_type = read_byte(reader)?;
                let list_len = i32::from_be_bytes([read_byte(reader)?; 4]);
                if list_len < 1 {
                    return Ok(Tag::List(vec![Tag::End]));
                }
                let mut list_elements = vec![];
                for _ in 0..list_len {
                    list_elements.push(read_tag(reader)?);
                }
                return Ok(Tag::List(list_elements))
            }
            0x0A => {
                let mut compound_elements = vec![];
                loop {
                    let tag = read_named_tag(reader)?;
                    if tag.tag == Tag::End {
                        break;
                    }
                    else {
                        compound_elements.push(tag);
                    }
                }
                return Ok(Tag::Compound(compound_elements));
            }
            0x0B => {
                let array_len = i32::from_be_bytes([read_byte(reader)?; 4]);
                let mut array = vec![];
                for _ in 0..array_len {
                    array.push(i32::from_be_bytes([read_byte(reader)?; 4]));
                }
                return Ok(Tag::IntArray(array));
            }
            0x0C => {
                let array_len = i32::from_be_bytes([read_byte(reader)?; 4]);
                let mut array = vec![];
                for _ in 0..array_len {
                    array.push(i64::from_be_bytes([read_byte(reader)?; 8]));
                }
                return Ok(Tag::LongArray(array));
            }
            _ => {
                return Err(Error::InvalidNBTType);
            }
        }
    }
    #[derive(PartialEq, Clone, Debug)]
    /// Represents a value in a NBT structure.
    pub enum Tag {
        /// A signed byte.
        Byte(i8),
        /// A Java Short.
        Short(i16),
        /// A Java Int.
        Int(i32),
        /// A Java Long.
        Long(i64),
        /// A Java Float.
        Float(f32),
        /// A Java Double.
        Double(f64),
        /// An array of signed bytes.
        ByteArray(Vec<i8>),
        /// A Java modified UTF-8 string.
        String(String),
        /// A list type containing a list of tags without names. All tags will be of the same type.
        List(Vec<Tag>),
        /// A compound type containing a list of named tags.
        Compound(Vec<NamedTag>),
        /// An array of Java Ints.
        IntArray(Vec<i32>),
        /// An array of Java Longs.
        LongArray(Vec<i64>),
        /// Represents the end of a compound or list tag.
        End
    }
    impl Tag {
        fn tag_prefix(self) -> u8 {
            match self {
                Self::End => 0,
                Self::Byte(_) => 1,
                Self::Short(_) => 2,
                Self::Int(_) => 3,
                Self::Long(_) => 4,
                Self::Float(_) => 5,
                Self::Double(_) => 6,
                Self::ByteArray(_) => 7,
                Self::String(_) => 8,
                Self::List(_) => 9,
                Self::Compound(_) => 10,
                Self::IntArray(_) => 11,
                Self::LongArray(_) => 12
            }
        }
        /// Writes this tag to a series of bytes. Does not include the tag's type ID prefix. Does
        /// include list and compound tag's ending byte.
        pub fn write_to_bytes(self) -> Result<Vec<u8>, Error> {
            match self {
                // The end tag has no data.
                Self::End => return Ok(vec![]),
                // It would be great to compact these as they use similar footprints, but the
                // different data types prevent doing this practically.
                Self::Byte(data) => {
                    return Ok(data.to_be_bytes().to_vec());
                },
                Self::Short(data) => {
                    return Ok(data.to_be_bytes().to_vec());
                },
                Self::Int(data) => {
                    return Ok(data.to_be_bytes().to_vec());
                },
                Self::Long(data) => {
                    return Ok(data.to_be_bytes().to_vec());
                },
                Self::Float(data) => {
                    return Ok(data.to_be_bytes().to_vec());
                },
                Self::Double(data) => {
                    return Ok(data.to_be_bytes().to_vec());
                },
                Self::ByteArray(data) => {
                    let len_prefix = data.len() as i32;
                    let mut final_data = vec![];
                    for byte in &len_prefix.to_be_bytes() {
                        final_data.push(*byte);
                    }
                    for byte in data {
                        final_data.push(byte.to_be_bytes()[0]);
                    }
                    return Ok(final_data);
                },
                Self::IntArray(data) => {
                    let len_prefix = data.len() as i32;
                    let mut final_data = vec![];
                    for byte in &len_prefix.to_be_bytes() {
                        final_data.push(*byte);
                    }
                    for chunk in data {
                        for byte in &chunk.to_be_bytes() {
                            final_data.push(*byte);
                        }
                    }
                    return Ok(final_data);
                },
                Self::LongArray(data) => {
                    let len_prefix = data.len() as i32;
                    let mut final_data = vec![];
                    for byte in &len_prefix.to_be_bytes() {
                        final_data.push(*byte);
                    }
                    for chunk in data {
                        for byte in &chunk.to_be_bytes() {
                            final_data.push(*byte);
                        }
                    }
                    return Ok(final_data);
                },
                Self::String(data) => {
                    let mut final_data = vec![];
                    let strbytes = data.as_bytes();
                    for byte in &(strbytes.len() as u16).to_be_bytes() {
                        final_data.push(*byte);
                    }
                    for byte in strbytes {
                        final_data.push(*byte);
                    }
                    return Ok(final_data);
                },
                Self::List(data) => {
                    let mut final_data = vec![];
                    final_data.push(data[0].clone().tag_prefix());
                    for byte in &(data.len() as i32).to_be_bytes() {
                        final_data.push(*byte);
                    }
                    for element in data {
                        for byte in element.write_to_bytes()? {
                            final_data.push(byte);
                        }
                    }
                    final_data.push(0x00);
                    return Ok(final_data);
                },
                Self::Compound(data) => {
                    let mut final_data = vec![];
                    for named_tag in data {
                        final_data.push(named_tag.tag.clone().tag_prefix());
                        let name_bytes = named_tag.name.as_bytes();
                        for byte in &(name_bytes.len() as u16).to_be_bytes() {
                            final_data.push(*byte);
                        }
                        for byte in name_bytes {
                            final_data.push(*byte);
                        }
                        for byte in named_tag.tag.write_to_bytes()? {
                            final_data.push(byte);
                        }
                    }
                    final_data.push(0x00);
                    return Ok(final_data);
                }
            }
        }
    }

    #[derive(PartialEq, Clone, Debug)]
    /// Represents a key-value pair in a NBT structure.
    pub struct NamedTag {
        /// Name of the given tag.
        pub name: String,
        /// Tag of this pair.
        pub tag: Tag
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Represents a namespaced selector.
pub struct Identifier {
    namespace: String,
    selector: String
}

impl Identifier {
    /// Creates a new Identifier using a stream of bytes. Returns how many bytes were used.
    pub fn from_bytes(bytes: &[u8]) -> Result<(Identifier, usize), Error> {
        let raw_parts = generalized::string_from_bytes(bytes)?;
        return Ok((Identifier::from_string(raw_parts.0)?, raw_parts.1));
    }
    /// Creates a new Identifier from a Read type.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Identifier, Error> {
        return Ok(Identifier::from_string(generalized::string_from_reader(reader)?)?);
    }
    /// Creates a new Identifier from a String.
    pub fn from_string(string: String) -> Result<Identifier, Error> {
        let mut whole_chunks = vec![];
        for chunk in string.split(":") {
            whole_chunks.push(chunk);
        }
        if whole_chunks.len() > 2 {
            return Err(Error::InvalidIdentifier);
        }
        else if whole_chunks.len() < 2 {
            return Ok(Identifier {
                namespace: String::from("minecraft"),
                selector: String::from(whole_chunks[0])
            });
        }
        else {
            return Ok(Identifier {
                namespace: String::from(whole_chunks[0]),
                selector: String::from(whole_chunks[1])
            });
        }
    }
    /// Writes this Identifier to a series of bytes.
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        return Ok(generalized::string_to_bytes(self.to_string()?)?);
    }
    /// Writes this Identifier to a Write type.
    pub fn to_writer<W: std::io::Write>(self, writer: &mut W) -> Result<(), Error> {
        generalized::string_to_writer(writer, self.to_string()?)?;
        return Ok(());
    }
    /// Writes this Identifier to a String. Always writes in the extended format for selectors under
    /// the `minecraft` namespace.
    pub fn to_string(self) -> Result<String, Error> {
        let mut full_string = String::new();
        full_string += &self.namespace;
        full_string += ":";
        full_string += &self.selector;
        return Ok(full_string);
    }
    /// Get the namespace of this Identifier. This is the part before the colon.
    pub fn get_namespace(self) -> String {
        return self.namespace;
    }
    /// Get the selector of this Identifier. This is the part after the colon.
    pub fn get_selector(self) -> String {
        return self.selector;
    }
}

use std::f64::consts::PI;
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Represents an angle. Cannot be greater than one full rotation, does not have negative values.
pub struct Angle {
    value: u8
}

impl Angle {
    /// Creates a new `Angle` using a byte. The byte is expected to reperesent how many 256ths of a
    /// full turn this angle represents. Always uses a single byte.
    pub fn from_bytes(bytes: &[u8]) -> Result<(Angle, usize), Error> {
        if bytes.len() < 1 {
            return Err(Error::MissingData);
        }
        return Ok((Angle { value: bytes[0] }, 1));
    }
    /// Creates a new `Angle` that is the given amount of degrees. Absoulte value is taken for
    /// negative values. Values over a full turn have the amount of turns discarded. Some
    /// significant precision is lost switching to Minecraft's format.
    pub fn from_degrees(degrees: f64) -> Angle {
        let mut workable = degrees;
        if workable < 0.0 {
            workable = workable * -1.0;
        }
        while workable > 360.0 {
            workable -= 360.0;
        }
        return Angle {
            value: ((workable / 360.0) * 256.0) as u8
        };
    }
    /// Creates a new `Angle` that is the given amount of radians. Absoulte value is taken for
    /// negative values. Values over a full turn have the amount of turns discarded. Some
    /// significant precision is lost switching to Minecraft's format.
    pub fn from_radians(radians: f64) -> Angle {
        let mut workable = radians;
        if workable < 0.0 {
            workable = workable * -1.0;
        }
        while workable > 2.0 * PI {
            workable -= 2.0 * PI;
        }
        return Angle {
            value: ((workable / (2.0 * PI)) * 256.0) as u8
        };
    }
    /// Returns how many 256ths of a full turn this angle represents. This is the data's actual
    /// format, and the most exact representation.
    pub fn as_256ths(self) -> u8 {
        return self.value;
    }
    /// Returns how many degrees this angle represents.
    pub fn to_degrees(self) -> f64 {
        return ((self.as_256ths() as f64) / 256.0) * 360.0;
    }
    /// Returns how many radians this angle represents.
    pub fn to_radians(self) -> f64 {
        return self.to_degrees() * (PI/180.0);
    }
    /// Encodes this angle as a byte representing how many 256ths of a full turn this angle is.
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        return Ok(vec![self.value]);
    }
}

/// Represents a Java Int (i32) using between 1-5 bytes.
#[derive(Eq, Clone, Copy, Debug)]
pub struct VarInt {
    value: i32,
    length: u8
}

impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VarInt {{ {:?} }}", self.value)
    }
}

impl PartialEq for VarInt {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {
            return true;
        }
        else {
            return false;
        }
    }
}

impl VarInt {
    /// Returns the value of a given VarInt
    pub fn value(self) -> i32 {
        return self.value;
    }
    /// Creates a VarInt from a series of bytes. Returns the value and the amount of bytes used if
    /// creation is successful.
    pub fn from_bytes(data: &[u8]) -> Result<(VarInt, usize), Error> {
        let mut iterator = data.iter();
        let mut result = 0;

        let msb: u8 = 0b10000000;
        let mask: u8 = !msb;

        for i in 0..5 {
            let read;
            match iterator.next() {
                Some(val) => {
                    read = val;
                }
                None => {
                    return Err(Error::MissingData);
                }
            }

            result |= ((read & mask) as i32) << (7 * i);

            // The 5th byte is only allowed to have the 4 smallest bits set
            if i == 4 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }

            if (read & msb) == 0 {
                return Ok((VarInt {value: result, length: i}, i as usize));
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::from_bytes reached end of function, which should not be possible");
    }
    /// Creates a VarInt from a reader containing bytes.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<VarInt, Error> {
        let mut result = 0;

        let msb: u8 = 0b10000000;
        let mask: u8 = !msb;
    
        for i in 0..5 {
            let read = read_byte(reader)?;
    
            result |= ((read & mask) as i32) << (7 * i);
    
            // The 5th byte is only allowed to have the 4 smallest bits set
            if i == 4 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }
    
            if (read & msb) == 0 {
                return Ok(VarInt {value: result, length: i});
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::from_reader reached end of function, which should not be possible");
    }
    /// Writes a VarInt to a writer as a series of bytes.
    pub fn to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), Error> {
        let msb: u8 = 0b10000000;
        let mask: i32 = 0b01111111;
        let mut val = self.value;

        for _ in 0..5 {
            let tmp = (val & mask) as u8;
            val &= !mask;
            val = val.rotate_right(7);

            if val != 0 {
                match writer.write_all(&[tmp | msb]) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::WriterError(e));
                    }
                }
            } else {
                match writer.write_all(&[tmp]) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::WriterError(e));
                    }
                }
                return Ok(());
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::to_writer reached end of function, which should not be possible");
    } 
    /// Converts a VarInt to a series of bytes.
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        let msb: u8 = 0b10000000;
        let mask: i32 = 0b01111111;
        let mut val = self.value;

        for _ in 0..5 {
            let tmp = (val & mask) as u8;
            val &= !mask;
            val = val.rotate_right(7);

            if val != 0 {
                bytes.push(tmp | msb);
            } else {
                bytes.push(tmp);
                return Ok(bytes);
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::to_bytes reached end of function, which should not be possible");
    }
    /// Creates a VarInt from a given value.
    pub fn from_value(value: i32) -> Result<VarInt, Error> {
        Ok(VarInt {
            value,
            length: VarInt::get_len_from_value(value)?
        })
    }
    fn get_len_from_value(value: i32) -> Result<u8, Error> {
        Ok(VarInt { value, length: 0 }.to_bytes()?.len() as u8)
    }
}


/// Represents a Java Long (i64) using between 1-10 bytes.
#[derive(Eq, Clone, Copy, Debug)]
pub struct VarLong {
    value: i64,
    length: u8
}

impl std::fmt::Display for VarLong {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VarLong {{ {:?} }}", self.value)
    }
}

impl PartialEq for VarLong {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {
            return true;
        }
        else {
            return false;
        }
    }
}

impl VarLong {
    /// Returns the value of a given VarInt
    pub fn value(self) -> i64 {
        return self.value;
    }
    /// Creates a VarLong from a series of bytes. Returns the value and the amount of bytes used if
    /// creation is successful.
    pub fn from_bytes(data: &[u8]) -> Result<(VarInt, usize), Error> {
        let mut iterator = data.iter();
        let mut result = 0;

        let msb: u8 = 0b10000000;
        let mask: u8 = !msb;

        for i in 0..10 {
            let read;
            match iterator.next() {
                Some(val) => {
                    read = val;
                }
                None => {
                    return Err(Error::MissingData);
                }
            }

            result |= ((read & mask) as i32) << (7 * i);

            // The 10th byte is only allowed to have the 4 smallest bits set
            if i == 9 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }

            if (read & msb) == 0 {
                return Ok((VarInt {value: result, length: i}, i as usize));
            }
        }
        // This will never occur.
        panic!("golden_apple::VarLong::from_bytes reached end of function, which should not be possible");
    }
    /// Creates a VarLong from a reader containing bytes.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<VarInt, Error> {
        let mut result = 0;

        let msb: u8 = 0b10000000;
        let mask: u8 = !msb;
    
        for i in 0..10 {
            let read = read_byte(reader)?;
    
            result |= ((read & mask) as i32) << (7 * i);
    
            // The 10th byte is only allowed to have the 4 smallest bits set
            if i == 9 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }
    
            if (read & msb) == 0 {
                return Ok(VarInt {value: result, length: i});
            }
        }
        // This will never occur.
        panic!("golden_apple::VarLong::from_reader reached end of function, which should not be possible");
    }
    /// Writes a VarLong to a writer as a series of bytes.
    pub fn to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), Error> {
        let msb: u8 = 0b10000000;
        let mask: i64 = 0b01111111;
        let mut val = self.value;

        for _ in 0..5 {
            let tmp = (val & mask) as u8;
            val &= !mask;
            val = val.rotate_right(7);

            if val != 0 {
                match writer.write_all(&[tmp | msb]) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::WriterError(e));
                    }
                }
            } else {
                match writer.write_all(&[tmp]) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::WriterError(e));
                    }
                }
                return Ok(());
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::to_writer reached end of function, which should not be possible");
    } 
    /// Converts a VarLong to a series of bytes.
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        let msb: u8 = 0b10000000;
        let mask: i64 = 0b01111111;
        let mut val = self.value;

        for _ in 0..10 {
            let tmp = (val & mask) as u8;
            val &= !mask;
            val = val.rotate_right(7);

            if val != 0 {
                bytes.push(tmp | msb);
            } else {
                bytes.push(tmp);
                return Ok(bytes);
            }
        }
        // This will never occur.
        panic!("golden_apple::VarLong::to_bytes reached end of function, which should not be possible");
    }
    /// Creates a VarLong from a given value.
    pub fn from_value(value: i64) -> Result<VarLong, Error> {
        Ok(VarLong {
            value,
            length: VarLong::get_len_from_value(value)?
        })
    }
    fn get_len_from_value(value: i64) -> Result<u8, Error> {
        Ok(VarLong { value, length: 0 }.to_bytes()?.len() as u8)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
/// Represents a position in the Minecraft world. Not the floating point values used for player
/// movement, but the whole number values used for things like block positions.
pub struct Position {
    // 26 bits for x and z, rounds up to 32
    // 12 for y rounds up to 16
    x: i32,
    y: i16,
    z: i32
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Position {{ {}, {}, {} }}", self.x, self.y, self.z)
    }
}

impl Position {
    /// Returns the x coordinate of this Position.
    pub fn get_x(self) -> i32 {
        return self.x;
    }
    /// Returns the y coordinate of this Position.
    pub fn get_y(self) -> i16 {
        return self.y;
    }
    /// Returns the z coordinate of this Position.
    pub fn get_z(self) -> i32 {
        return self.z
    }
    /// Creates a Position from a series of bytes. Requires 8 bytes or more in the buffer. Also
    /// returns how many bytes were used in this function, which should always be 8.
    pub fn from_bytes(data: &[u8]) -> Result<(Position, usize), Error> {
        if data.len() < 8 {
            return Err(Error::MissingData);
        }

        let mut toconvert = [0; 8];
        let indexable_data = data.split_at(8).0;
        for i in 0..8 {
            toconvert[i] = indexable_data[i]
        }
        // convert to one big u64
        let u64val = u64::from_be_bytes(toconvert);

        // strip out values with bitmasks
        let mut x = (u64val >> 38) as i32;
        let mut y = (u64val & 0xfff) as i16;
        let mut z = (u64val << 26 >> 38) as i32;

        // convert to negative if appropriate
        if x >= 2^25 {
            x -= 2^26;
        }
        if y >= 2^11 {
            y -= 2^12;
        }
        if z >= 2^25 {
            z -= 2^26
        }
        return Ok((Position { x, y, z }, 8));
    }
    /// Creates a Position from a Read type.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Position, Error> {
        let mut toconvert = [0; 8];
        for i in 0..8 {
            toconvert[i] = read_byte(reader)?;
        }
        let u64val = u64::from_be_bytes(toconvert);

        // strip out values with bitmasks
        let mut x = (u64val >> 38) as i32;
        let mut y = (u64val & 0xfff) as i16;
        let mut z = (u64val << 26 >> 38) as i32;

        // convert to negative if appropriate
        if x >= 2^25 {
            x -= 2^26;
        }
        if y >= 2^11 {
            y -= 2^12;
        }
        if z >= 2^25 {
            z -= 2^26
        }
        return Ok(Position { x, y, z });
    }
    /// Creates a Position from coordinate values.
    pub fn from_values(x: i32, y: i16, z: i32) -> Position {
        Position {
            x, y, z
        }
    }
    /// Converts a Position into a series of bytes.
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        let xval;
        let yval;
        let zval;
        if self.x < 0 {
            xval = (self.x + (2^26)) as u64;
        }
        else {
            xval = self.x as u64;
        }
        if self.z < 0 {
            zval = (self.x + (2^26)) as u64;
        }
        else {
            zval = self.z as u64;
        }
        if self.y < 0 {
            yval = (self.y + (2^12)) as u64;
        }
        else {
            yval = self.y as u64;
        }
        let u64val: u64 = ((xval & 0x3FFFFFF) << 38) | ((zval & 0x3FFFFFF) << 12) | (yval & 0xFFF);
        let u64bytes = u64val.to_be_bytes();
        return Ok(u64bytes.to_vec());
    }
    /// Writes a Position to a Write type.
    pub fn to_writer<W: std::io::Write>(self, writer: &mut W) -> Result<(), Error> {
        let u64val: u64 = ((self.x as u64 & 0x3FFFFFF) << 38) | ((self.z as u64 & 0x3FFFFFF) << 12) | (self.y as u64 & 0xFFF);
        let u64bytes = u64val.to_be_bytes();
        match writer.write_all(&u64bytes) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
}

/// `generalized` contains many repetetive and unnecisary functions for reading and writing data.
/// For sake of completion and inclusiveness, all standard types that may be written over the
/// stream, no matter how easy to parse, are included here.
pub mod generalized {
    use super::Error;
    use super::read_byte;
    use super::VarInt;

    /// Reads a `String` from a type implimenting `Read`. This function returns the string without the
    /// VarInt length prefix, and does not verify that the text is utf8.
    pub fn string_from_reader<R: std::io::Read>(reader: &mut R) -> Result<String, Error> {
        let string_len = super::VarInt::from_reader(reader)?.value();
        let mut text: Vec<u8> = vec![0; string_len as usize];
        match reader.read_exact(&mut text) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::ReaderError(e));
            }
        }
        unsafe {
            // Minecraft is known to put weird stuff in their strings, so we're not going to double check.
            return Ok(String::from_utf8_unchecked(text));
        }
    }
    /// Reads a `String` from a series of bytes. This function returns the string without the VarInt
    /// length prefix, but does include the size of that VarInt in the final size calculation. The text
    /// is not verified to be utf8.
    pub fn string_from_bytes(bytes: &[u8]) -> Result<(String, usize), Error> {
        let string_len = super::VarInt::from_bytes(bytes)?;
        let mut text: Vec<u8> = vec![0; string_len.0.value() as usize];
        let finbytes = bytes.split_at(string_len.1).1;
        for i in 0..text.len() {
            text[i] = finbytes[i];
        }
        unsafe {
            // Minecraft is known to put weird stuff in their strings, so we're not going to double check.
            return Ok((String::from_utf8_unchecked(text), string_len.0.value() as usize + string_len.1));
        }
    }
    /// Writes a `String` to a Write interface.
    pub fn string_to_writer<W: std::io::Write>(writer: &mut W, data: String) -> Result<(), Error> {
        let as_bytes = data.as_bytes();
        let length_prefix = VarInt::from_value(as_bytes.len() as i32)?;
        match writer.write_all(&length_prefix.to_bytes()?) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
        match writer.write_all(as_bytes) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
        return Ok(());
    }
    /// Converts a `String` to a VarInt length prefixed series of bytes.
    pub fn string_to_bytes(data: String) -> Result<Vec<u8>, Error> {
        let as_bytes = data.as_bytes();
        let length_prefix = VarInt::from_value(as_bytes.len() as i32)?;
        let mut vec_vals = as_bytes.to_vec();
        for byte in length_prefix.to_bytes()? {
            vec_vals.push(byte);
        }
        return Ok(vec_vals);
    }
    /// Woefully unnessicary. Seriously, bools are just 0x00 or 0x01.
    pub fn boolean_from_reader<R: std::io::Read>(reader: &mut R) -> Result<bool, Error> {
        let byte = read_byte(reader)?;
        if byte == 0x00 {
            return Ok(false);
        }
        if byte == 0x01 {
            return Ok(true);
        }
        return Err(Error::InvalidBool);
    }
    /// Woefully unnessicary. Seriously, bools are just 0x00 or 0x01.
    /// Side note: this function will always read just a single byte, making half of the
    /// return type pointless.
    pub fn boolean_from_bytes(bytes: &[u8]) -> Result<(bool, usize), Error> {
        if bytes.len() < 1 {
            return Err(Error::MissingData);
        }
        if bytes[0] == 0x00 {
            return Ok((false, 1));
        }
        if bytes[0] == 0x01 {
            return Ok((true, 1));
        }
        return Err(Error::InvalidBool);
    }
    /// Either writes 0x00 or 0x01 to the writer. Come on, you don't need this.
    pub fn boolean_to_writer<W: std::io::Write>(writer: &mut W, data: bool) -> Result<(), Error> {
        if data {
            match writer.write_all(&[0x01]) {
                Ok(_) => {},
                Err(e) => {
                    return Err(Error::WriterError(e));
                }
            }
        }
        else {
            match writer.write_all(&[0x00]) {
                Ok(_) => {},
                Err(e) => {
                    return Err(Error::WriterError(e));
                }
            }
        }
        return Ok(());
    }
    /// What's wrong with you? This isn't something you should need or use. It's one byte. It's not
    /// even possible to get an error here.
    pub fn boolean_to_bytes(data: bool) -> Result<Vec<u8>, Error> {
        if data {
            return Ok(vec![0x01]);
        }
        else {
            return Ok(vec![0x00]);
        }
    }
    /// Uses a Read type to read a Java Byte from the stream.
    pub fn byte_from_reader<R: std::io::Read>(reader: &mut R) -> Result<i8, Error> {
        let byte = read_byte(reader)?;
        return Ok(i8::from_be_bytes([byte]));
    }
    /// Reads a Java Byte from a list of bytes. Returns the value and number of bytes read.
    pub fn byte_from_bytes(bytes: &[u8]) -> Result<(i8, usize), Error> {
        if bytes.len() < 1 {
            return Err(Error::MissingData);
        }
        return Ok((i8::from_be_bytes([bytes[0]]), 1));
    }
    /// Writes a Java Byte to a Write type.
    pub fn byte_to_writer<W: std::io::Write>(writer: &mut W, byte: i8) -> Result<(), Error> {
        match writer.write_all(&byte.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Byte as an array of bytes.
    pub fn byte_to_bytes(byte: i8) -> Result<Vec<u8>, Error> {
        return Ok(byte.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read an unsigned Java Byte from the stream.
    pub fn unsigned_byte_from_reader<R: std::io::Read>(reader: &mut R) -> Result<u8, Error> {
        let byte = read_byte(reader)?;
        return Ok(u8::from_be_bytes([byte]));
    }
    /// Reads an unsigned Java Byte from a list of bytes. Returns the value and number of bytes read.
    pub fn unsigned_byte_from_bytes(bytes: &[u8]) -> Result<(u8, usize), Error> {
        if bytes.len() < 1 {
            return Err(Error::MissingData);
        }
        return Ok((u8::from_be_bytes([bytes[0]]), 1));
    }
    /// Writes an unsigned Java Byte to a Write type.
    pub fn unsigned_byte_to_writer<W: std::io::Write>(writer: &mut W, byte: u8) -> Result<(), Error> {
        match writer.write_all(&byte.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns an unsigned Java Byte as an array of bytes.
    pub fn unsigned_byte_to_bytes(byte: u8) -> Result<Vec<u8>, Error> {
        return Ok(byte.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Short from the stream.
    pub fn short_from_reader<R: std::io::Read>(reader: &mut R) -> Result<i16, Error> {
        let bytes = [read_byte(reader)?, read_byte(reader)?];
        return Ok(i16::from_be_bytes(bytes));
    }
    /// Reads a Java Short from a list of bytes. Returns the value and number of bytes read.
    pub fn short_from_bytes(bytes: &[u8]) -> Result<(i16, usize), Error> {
        if bytes.len() < 2 {
            return Err(Error::MissingData);
        }
        return Ok((i16::from_be_bytes([bytes[0], bytes[1]]), 2));
    }
    /// Writes a Java Short to a Write type.
    pub fn short_to_writer<W: std::io::Write>(writer: &mut W, short: i16) -> Result<(), Error> {
        match writer.write_all(&short.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Short as an array of bytes.
    pub fn short_to_bytes(short: i16) -> Result<Vec<u8>, Error> {
        return Ok(short.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read an unsigned Java Short from the stream.
    pub fn unsigned_short_from_reader<R: std::io::Read>(reader: &mut R) -> Result<u16, Error> {
        let bytes = [read_byte(reader)?, read_byte(reader)?];
        return Ok(u16::from_be_bytes(bytes));
    }
    /// Reads an unsigned Java Short from a list of bytes. Returns the value and number of bytes read.
    pub fn unsigned_short_from_bytes(bytes: &[u8]) -> Result<(u16, usize), Error> {
        if bytes.len() < 2 {
            return Err(Error::MissingData);
        }
        return Ok((u16::from_be_bytes([bytes[0], bytes[1]]), 2));
    }
    /// Writes an unsigned Java Short to a Write type.
    pub fn unsigned_short_to_writer<W: std::io::Write>(writer: &mut W, short: u16) -> Result<(), Error> {
        match writer.write_all(&short.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns an unsigned Java Short as an array of bytes.
    pub fn unsigned_short_to_bytes(short: u16) -> Result<Vec<u8>, Error> {
        return Ok(short.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Int from the stream.
    pub fn int_from_reader<R: std::io::Read>(reader: &mut R) -> Result<i32, Error> {
        let bytes = [read_byte(reader)?; 4];
        return Ok(i32::from_be_bytes(bytes));
    }
    /// Reads a Java Int from a list of bytes. Returns the value and number of bytes read.
    pub fn int_from_bytes(bytes: &[u8]) -> Result<(i32, usize), Error> {
        if bytes.len() < 4 {
            return Err(Error::MissingData);
        }
        return Ok((i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 4));
    }
    /// Writes a Java Int to a Write type.
    pub fn int_to_writer<W: std::io::Write>(writer: &mut W, int: i32) -> Result<(), Error> {
        match writer.write_all(&int.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Int as an array of bytes.
    pub fn int_to_bytes(int: i32) -> Result<Vec<u8>, Error> {
        return Ok(int.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Long from the stream.
    pub fn long_from_reader<R: std::io::Read>(reader: &mut R) -> Result<i64, Error> {
        let bytes = [read_byte(reader)?; 8];
        return Ok(i64::from_be_bytes(bytes));
    }
    /// Reads a Java Long from a list of bytes. Returns the value and number of bytes read.
    pub fn long_from_bytes(bytes: &[u8]) -> Result<(i64, usize), Error> {
        if bytes.len() < 8 {
            return Err(Error::MissingData);
        }
        return Ok((i64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]), 8));
    }
    /// Writes a Java Long to a Write type.
    pub fn long_to_writer<W: std::io::Write>(writer: &mut W, long: i64) -> Result<(), Error> {
        match writer.write_all(&long.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Long as an array of bytes.
    pub fn long_to_bytes(long: i64) -> Result<Vec<u8>, Error> {
        return Ok(long.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Float from the stream.
    pub fn float_from_reader<R: std::io::Read>(reader: &mut R) -> Result<f32, Error> {
        let bytes = [read_byte(reader)?; 4];
        return Ok(f32::from_be_bytes(bytes));
    }
    /// Reads a Java Float from a list of bytes. Returns the value and number of bytes read.
    pub fn float_from_bytes(bytes: &[u8]) -> Result<(f32, usize), Error> {
        if bytes.len() < 4 {
            return Err(Error::MissingData);
        }
        return Ok((f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 4));
    }
    /// Writes a Java Float to a Write type.
    pub fn float_to_writer<W: std::io::Write>(writer: &mut W, float: f32) -> Result<(), Error> {
        match writer.write_all(&float.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Float as an array of bytes.
    pub fn float_to_bytes(float: f32) -> Result<Vec<u8>, Error> {
        return Ok(float.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Double from the stream.
    pub fn double_from_reader<R: std::io::Read>(reader: &mut R) -> Result<f64, Error> {
        let bytes = [read_byte(reader)?; 8];
        return Ok(f64::from_be_bytes(bytes));
    }
    /// Reads a Java Double from a list of bytes. Returns the value and number of bytes read.
    pub fn double_from_bytes(bytes: &[u8]) -> Result<(f64, usize), Error> {
        if bytes.len() < 8 {
            return Err(Error::MissingData);
        }
        return Ok((f64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]), 8));
    }
    /// Writes a Java Double to a Write type.
    pub fn double_to_writer<W: std::io::Write>(writer: &mut W, double: f64) -> Result<(), Error> {
        match writer.write_all(&double.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Double as an array of bytes.
    pub fn double_to_bytes(double: f64) -> Result<Vec<u8>, Error> {
        return Ok(double.to_be_bytes().to_vec());
    }
}

fn read_byte<R: std::io::Read>(reader: &mut R) -> Result<u8, Error> {
    let mut read: [u8; 1] = [0x00];
    match reader.read_exact(&mut read) {
        Ok(_) => {
            return Ok(read[0]);
        },
        Err(e) => {
            return Err(Error::ReaderError(e));
        }
    }
}

mod test {
    #[test]
    fn varint_standard_values() -> Result<(), super::Error> {
        use super::VarInt;
        // Create the list of standard values
        let val_0 = VarInt::from_value(0)?;
        let val_1 = VarInt::from_value(1)?;
        let val_largest_num = VarInt::from_value(2147483647)?;
        let val_minus_one = VarInt::from_value(-1)?;
        let val_smallest_num = VarInt::from_value(-2147483648)?;

        // Check that the values are still the same
        assert_eq!(val_0.value(), 0);
        assert_eq!(val_1.value(), 1);
        assert_eq!(val_largest_num.value(), 2147483647);
        assert_eq!(val_minus_one.value(), -1);
        assert_eq!(val_smallest_num.value(), -2147483648);

        // Check that encoding works properly
        assert_eq!(val_0.to_bytes()?, [0x00]);
        assert_eq!(val_1.to_bytes()?, [0x01]);
        assert_eq!(val_largest_num.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0x07]);
        assert_eq!(val_minus_one.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0x0f]);
        assert_eq!(val_smallest_num.to_bytes()?, [0x80, 0x80, 0x80, 0x80, 0x08]);

        // Check that decoding works properly
        assert_eq!(val_0.value(), VarInt::from_bytes(&[0x00])?.0.value());
        assert_eq!(val_1.value(), VarInt::from_bytes(&[0x01])?.0.value());
        assert_eq!(val_largest_num.value(), VarInt::from_bytes(&[0xff, 0xff, 0xff, 0xff, 0x07])?.0.value());
        assert_eq!(val_minus_one.value(), VarInt::from_bytes(&[0xff, 0xff, 0xff, 0xff, 0x0f])?.0.value());
        assert_eq!(val_smallest_num.value(), VarInt::from_bytes(&[0x80, 0x80, 0x80, 0x80, 0x08])?.0.value());
        return Ok(());
    }
    #[test]
    fn varlong_standard_values() -> Result<(), super::Error> {
        use super::VarLong;
        // Create the list of standard values
        let val_0 = VarLong::from_value(0)?;
        let val_1 = VarLong::from_value(1)?;
        let val_largest_num = VarLong::from_value(9223372036854775807)?;
        let val_minus_one = VarLong::from_value(-1)?;
        let val_smallest_num = VarLong::from_value(-9223372036854775808)?;

        // Check that the values are still the same
        assert_eq!(val_0.value(), 0);
        assert_eq!(val_1.value(), 1);
        assert_eq!(val_largest_num.value(), 9223372036854775807);
        assert_eq!(val_minus_one.value(), -1);
        assert_eq!(val_smallest_num.value(), -9223372036854775808);

        // Check that encoding works properly
        assert_eq!(val_0.to_bytes()?, [0x00]);
        assert_eq!(val_1.to_bytes()?, [0x01]);
        assert_eq!(val_largest_num.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);
        assert_eq!(val_minus_one.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);
        assert_eq!(val_smallest_num.to_bytes()?, [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
        return Ok(());
    }
    #[test]
    fn position_standard_values() -> Result<(), super::Error> {
        use super::Position;
        // Create the list of standard values
        let zeroed = Position::from_values(0, 0, 0);
        let max_value = Position::from_values(i32::MAX, i16::MAX, i32::MAX);
        let min_value = Position::from_values(i32::MIN, i16::MIN, i32::MIN);

        // Check that the values are still the same
        assert_eq!(zeroed.get_x(), 0);
        assert_eq!(zeroed.get_y(), 0);
        assert_eq!(zeroed.get_z(), 0);
        assert_eq!(max_value.get_x(), i32::MAX);
        assert_eq!(max_value.get_y(), i16::MAX);
        assert_eq!(max_value.get_z(), i32::MAX);
        assert_eq!(min_value.get_x(), i32::MIN);
        assert_eq!(min_value.get_y(), i16::MIN);
        assert_eq!(min_value.get_z(), i32::MIN);

        // Check that encoding works properly
        assert_eq!(zeroed.to_bytes()?, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(max_value.to_bytes()?, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(min_value.to_bytes()?, [0x00, 0x00, 0x06, 0x00, 0x00, 0x01, 0x80, 0x0E]);
        return Ok(());
    }
    #[test]
    fn username_api() -> Result<(), super::Error> {
        use super::UUID;
        let uuid = UUID::from_username(String::from("thisjaiden"))?;
        assert_eq!(uuid.clone().to_value()?, 0x09773765901b4da1a1243467f482b8b3);
        assert_eq!(uuid.to_username()?, String::from("thisjaiden"));
        return Ok(());
    }
}
