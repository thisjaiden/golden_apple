//! # Overview
//! `golden_apple` is a library for decoding, encoding, and using common types found in Minecraft:
//! Java Edition.
//!
//! # Goals
//! - Provide a generalized format for sharing and using Minecraft's data types
//! - Simplify the decoding and encoding of network data
//! - Abstract away enums usually passed as numbers
//!
//! # Usage
//! Proprietary Minecraft types like `VarInt`, `VarLong`, and `Position` are a part of the top
//! level module. Types that can be fully represented in Rust have encoders/decoders under
//! `golden_apple::generalized`, in case it isn't striaghtforward to do so. All enums are under
//! the `golden_apple::enums` module.

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
pub mod enums;

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

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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

/// Provides tools for reading, writing, and managing NBT types.
pub mod nbt;
// Unit testing module.
mod test;
