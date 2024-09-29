use crate::{Error, Identifier, VarInt, UUID};
use crate::generalized::{
    boolean_from_reader, byte_from_reader, byte_to_bytes, int_to_bytes,
    long_to_bytes, string_from_reader_no_cesu8, string_to_bytes_no_cesu8,
    unsigned_byte_from_reader
};
use std::io::Read;

#[derive(Clone, PartialEq, Eq, Debug)]
/// A packet sent from the client to the server during the "configuration" phase.
pub enum ServerboundPacket {
    ClientInformation {
        locale: String,
        view_distance: i8,
        chat_mode: ChatSettings,
        chat_colors: bool,
        skin_parts: SkinSettings,
        // TODO: make this an enum
        main_hand: VarInt,
        text_filtering: bool,
        allow_server_listings: bool
    },
    CookieResponse {
        key: Identifier,
        payload: Option<Vec<u8>>
    },
    PluginMessage {
        channel: Identifier,
        data: Vec<u8>
    },
    AcknowledgeFinishConfiguration,
    KeepAlive {
        id: i64
    },
    Pong {
        id: i32
    },
    ResourcePackResponse {
        uuid: UUID,
        // TODO: make this an enum
        result: VarInt
    },
    KnownPacks {
        packs: Vec<KnownPack>
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// A packet sent from the server to the client during the "configuration" phase.
pub enum ClientboundPacket {
    
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>
}


impl ServerboundPacket {
    /// Converts this packet into bytes that can be sent over the network to a
    /// server using this protocol version.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut packet_bytes = self.to_most_bytes()?;
        // Calculate packet length, prepend, and send it!
        let packet_length = packet_bytes.len();
        let mut result = VarInt::from_value(packet_length as i32)?.to_bytes()?;
        result.append(&mut packet_bytes);

        Ok(result)
    }
    /// Converts the packet to bytes in the proper format for networking with
    /// traditional Minecraft software *minus* the packet length being prepended.
    fn to_most_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        match self {
            Self::ClientInformation {
                locale, view_distance, chat_mode,
                chat_colors, skin_parts, main_hand,
                text_filtering, allow_server_listings
            } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x00)?.to_bytes()?);

                // Payload
                assert!(locale.chars().count() <= 16);
                bytes.append(&mut string_to_bytes_no_cesu8(locale.clone())?);

                bytes.append(&mut byte_to_bytes(*view_distance)?);
                bytes.append(&mut chat_mode.to_varint().to_bytes()?);
                bytes.push(if *chat_colors { 0x01 } else { 0x00 });
                bytes.push(skin_parts.bits());
                bytes.append(&mut main_hand.to_bytes()?);
                bytes.push(if *text_filtering { 0x01 } else { 0x00 });
                bytes.push(if *allow_server_listings { 0x01 } else { 0x00 });
            }
            Self::CookieResponse { key, payload } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x01)?.to_bytes()?);

                // Payload
                bytes.append(&mut key.to_bytes()?);

                if let Some(payload) = payload {
                    bytes.push(0x01);

                    assert!(payload.len() <= 5120);
                    bytes.append(&mut VarInt::from_value(payload.len() as i32)?.to_bytes()?);
                    bytes.append(&mut payload.clone());
                }
                else {
                    bytes.push(0x00);
                }
            }
            Self::PluginMessage { channel, data } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x02)?.to_bytes()?);

                // Payload
                bytes.append(&mut channel.to_bytes()?);

                assert!(data.len() <= 32767);
                bytes.append(&mut data.clone());
            }
            Self::AcknowledgeFinishConfiguration => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x03)?.to_bytes()?);
            }
            Self::KeepAlive { id } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x04)?.to_bytes()?);

                // Payload
                bytes.append(&mut long_to_bytes(*id)?);
            }
            Self::Pong { id } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x05)?.to_bytes()?);

                // Payload
                bytes.append(&mut int_to_bytes(*id)?);
            }
            Self::ResourcePackResponse { uuid, result } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x06)?.to_bytes()?);
                
                // Payload
                bytes.append(&mut uuid.to_bytes()?);
                bytes.append(&mut result.to_bytes()?);
            }
            Self::KnownPacks { packs } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x07)?.to_bytes()?);

                // Payload
                bytes.append(&mut VarInt::from_value(packs.len() as i32)?.to_bytes()?);

                for pack in packs {
                    bytes.append(&mut string_to_bytes_no_cesu8(pack.namespace.clone())?);
                    bytes.append(&mut string_to_bytes_no_cesu8(pack.id.clone())?);
                    bytes.append(&mut string_to_bytes_no_cesu8(pack.version.clone())?);
                }
            }
        }

        Ok(bytes)
    }
    /// Converts this packet into bytes that can be sent over the network to a
    /// server using this protocol version, once compression has been enabled.
    /// Only use this method after recieving
    /// [crate::netty::login::ClientboundPacket::SetCompression]. Even if a
    /// packet isn't encrypted, the format is slightly different.
    // TODO: test that this is compliant and works
    pub fn to_bytes_com(&self, threshold: VarInt) -> Result<Vec<u8>, Error> {
        // Get packet data.
        let mut packet_bytes = self.to_most_bytes()?;
        // Calculate packet length.
        let packet_length = packet_bytes.len();

        // If it's below the packet compression threshold,
        if packet_length < threshold.value() as usize {
            // Prepend length and send it off!
            // We add 1 to `packet_length` to account for the compression length.
            // (which is zero, but encodes as one byte)
            let mut result = VarInt::from_value(packet_length as i32 + 1)?.to_bytes()?;
            // Insert the compression length (0)
            result.push(0x00);
            // Add the rest of the packet
            result.append(&mut packet_bytes);

            Ok(result)
        }
        else {
            // Otherwise, we need to compress the packet.
            use std::io::prelude::*;
            use flate2::Compression;
            use flate2::write::ZlibEncoder;
            // TODO: allow the user to select the compression type.
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
            // TODO: be more specific with the errors coming off of these `?`s.
            encoder.write_all(&packet_bytes)?;
            let mut compressed_data = encoder.finish()?;

            // Put the length of the compressed section of the packet into this VarInt
            let mut compressed_data_length = VarInt::from_value(compressed_data.len() as i32)?;
            compressed_data_length.calculate_read_size();

            // Prepend the value of (compressed data length + compressed data
            // length length).
            // Safe unwrap, since we just did `.calculate_read_size()`.
            let mut result = VarInt::from_value(
                compressed_data_length.value() +
                compressed_data_length.read_size().unwrap() as i32
            )?.to_bytes()?;
            // Prepend compressed data length
            result.append(&mut compressed_data_length.to_bytes()?);
            // Add the rest of the packet
            result.append(&mut compressed_data);

            Ok(result)
        }
    }
    /// Not done! Please wait for this to be finished or open a PR!
    #[cfg(feature = "encryption")]
    pub fn to_bytes_enc(&self) -> Result<Vec<u8>, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    #[cfg(feature = "encryption")]
    pub fn to_bytes_enc_com(&self, threshold: VarInt) -> Result<Vec<u8>, Error> {
        todo!()
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let packet_length = VarInt::from_reader(reader)?;
        
        Self::from_reader_internal(reader, packet_length)
    }
    fn from_reader_internal<R: Read>(reader: &mut R, packet_length: VarInt) -> Result<Self, Error> {
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00 => {
                let locale = string_from_reader_no_cesu8(reader)?;
                let view_distance = byte_from_reader(reader)?;
                let chat_mode = ChatSettings::try_from(VarInt::from_reader(reader)?)?;
                let chat_colors = boolean_from_reader(reader)?;
                let skin_parts = SkinSettings::from_bits_retain(unsigned_byte_from_reader(reader)?);
                let main_hand = VarInt::from_reader(reader)?;
                let text_filtering = boolean_from_reader(reader)?;
                let allow_server_listings = boolean_from_reader(reader)?;

                Ok(Self::ClientInformation {
                    locale, view_distance, chat_mode, chat_colors,
                    skin_parts, main_hand, text_filtering, allow_server_listings
                })
            }
            0x01 => {
                let key = Identifier::from_reader(reader)?;
                
                let payload = if boolean_from_reader(reader)? {
                    let len = VarInt::from_reader(reader)?;
                    let mut buf = vec![0x00; len.value() as usize];
                    reader.read_exact(&mut buf)?;
                    
                    Some(buf)
                }
                else { None };

                Ok(Self::CookieResponse { key, payload })
            }
            0x03 => Ok(Self::AcknowledgeFinishConfiguration),
            0x02 | 0x04..0x07 => todo!(),
            _ => Err(Error::InvalidPacketId(packet_id))
        }
    }
    /// Not done! Please wait for this to be finished or open a PR!
    #[cfg(feature = "encryption")]
    pub fn from_reader_enc<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
    /// Reads a packet from a [Read] type that is sent to a server using this
    /// protocol version. Expects that compression has been enabled. Only use
    /// this method after recieving
    /// [crate::netty::login::ClientboundPacket::SetCompression]. Even if a
    /// packet isn't encrypted, the format is slightly different.
    // TODO: test that this is compliant and works. This is pretty gross, could
    // use some cleanup too.
    pub fn from_reader_com<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let remaining_len = VarInt::from_reader(reader)?;
        let compressed_len = VarInt::from_reader(reader)?;
        if compressed_len.value() == 0 {
            // Packet is not compressed.
            Self::from_reader_internal(
                reader,
                VarInt::from_value(
                    remaining_len.value() -
                    compressed_len.read_size().unwrap() as i32
                )?
            )
        }
        else {
            // Packet is compressed. Grab all data...
            let mut packet_data = vec![0x00; remaining_len.value() as usize - compressed_len.read_size().unwrap() as usize];
            reader.read_exact(&mut packet_data)?;
            // Add a decoding wrapper...
            let mut decoded =
                flate2::bufread::ZlibDecoder::new(packet_data.as_ref());
            
            // And interpret the packet. Also return.
            Self::from_reader_internal(
                &mut decoded,
                VarInt::from_value(
                    remaining_len.value() -
                    compressed_len.read_size().unwrap() as i32
                )?
            )
        }
    }
    /// Not done! Please wait for this to be finished or open a PR!
    #[cfg(feature = "encryption")]
    pub fn from_reader_enc_com<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
}

impl ClientboundPacket {
    /// Converts the packet to bytes in the proper format for networking with
    /// traditional Minecraft software *minus* the packet length being prepended.
    fn to_most_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        match self {
            _ => todo!()
        }

        Ok(bytes)
    }
    /// Converts this packet into bytes that can be sent over the network to a
    /// client using this protocol version.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut packet_bytes = self.to_most_bytes()?;
        // Calculate packet length, prepend, and send it!
        let packet_length = packet_bytes.len();
        let mut result = VarInt::from_value(packet_length as i32)?.to_bytes()?;
        result.append(&mut packet_bytes);

        Ok(result)
    }
    /// Converts this packet into bytes that can be sent over the network to a
    /// client using this protocol version, once compression has been enabled.
    /// Only use this method after sending
    /// [crate::netty::login::ClientboundPacket::SetCompression]. Even if a
    /// packet isn't encrypted, the format is slightly different.
    // TODO: test that this is compliant and works
    pub fn to_bytes_com(&self, threshold: VarInt) -> Result<Vec<u8>, Error> {
        // Get packet data.
        let mut packet_bytes = self.to_most_bytes()?;
        // Calculate packet length.
        let packet_length = packet_bytes.len();

        // If it's below the packet compression threshold,
        if packet_length < threshold.value() as usize {
            // Prepend length and send it off!
            // We add 1 to `packet_length` to account for the compression length.
            // (which is zero, but encodes as one byte)
            let mut result = VarInt::from_value(packet_length as i32 + 1)?.to_bytes()?;
            // Insert the compression length (0)
            result.push(0x00);
            // Add the rest of the packet
            result.append(&mut packet_bytes);

            Ok(result)
        }
        else {
            // Otherwise, we need to compress the packet.
            use std::io::prelude::*;
            use flate2::Compression;
            use flate2::write::ZlibEncoder;
            // TODO: allow the user to select the compression type.
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
            // TODO: be more specific with the errors coming off of these `?`s.
            encoder.write_all(&packet_bytes)?;
            let mut compressed_data = encoder.finish()?;

            // Put the length of the compressed section of the packet into this VarInt
            let mut compressed_data_length = VarInt::from_value(compressed_data.len() as i32)?;
            compressed_data_length.calculate_read_size();

            // Prepend the value of (compressed data length + compressed data
            // length length).
            // Safe unwrap, since we just did `.calculate_read_size()`.
            let mut result = VarInt::from_value(
                compressed_data_length.value() +
                compressed_data_length.read_size().unwrap() as i32
            )?.to_bytes()?;
            // Prepend compressed data length
            result.append(&mut compressed_data_length.to_bytes()?);
            // Add the rest of the packet
            result.append(&mut compressed_data);

            Ok(result)
        }
    }
    /// Not done! Please wait for this to be finished or open a PR!
    #[cfg(feature = "encryption")]
    pub fn to_bytes_enc(&self) -> Result<Vec<u8>, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    #[cfg(feature = "encryption")]
    pub fn to_bytes_enc_com(&self, threshold: VarInt) -> Result<Vec<u8>, Error> {
        todo!()
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let packet_length = VarInt::from_reader(reader)?;
        
        Self::from_reader_internal(reader, packet_length)
    }
    fn from_reader_internal<R: Read>(reader: &mut R, packet_length: VarInt) -> Result<Self, Error> {
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00..0x10 => todo!(),
            _ => { Err(Error::InvalidPacketId(packet_id)) }
        }
    }
    /// Not done! Please wait for this to be finished or open a PR!
    #[cfg(feature = "encryption")]
    pub fn from_reader_enc<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
    /// Reads a packet from a [Read] type that is sent to a server using this
    /// protocol version. Expects that compression has been enabled. Only use
    /// this method after sending
    /// [crate::netty::login::ClientboundPacket::SetCompression]. Even if a
    /// packet isn't encrypted, the format is slightly different.
    // TODO: test that this is compliant and works. This is pretty gross, could
    // use some cleanup too.
    pub fn from_reader_com<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let remaining_len = VarInt::from_reader(reader)?;
        let compressed_len = VarInt::from_reader(reader)?;
        if compressed_len.value() == 0 {
            // Packet is not compressed. Return whatever standard parsing gives.
            Self::from_reader_internal(
                reader,
                VarInt::from_value(
                    remaining_len.value() -
                    compressed_len.read_size().unwrap() as i32
                )?
            )
        }
        else {
            // Packet is compressed. Grab all data...
            let mut packet_data = vec![0x00; remaining_len.value() as usize - compressed_len.read_size().unwrap() as usize];
            reader.read_exact(&mut packet_data)?;
            // Add a decoding wrapper...
            let mut decoded =
                flate2::bufread::ZlibDecoder::new(packet_data.as_ref());
            
            // And interpret the packet. Also return it.
            Self::from_reader_internal(
                &mut decoded,
                VarInt::from_value(
                    remaining_len.value() -
                    compressed_len.read_size().unwrap() as i32
                )?
            )
        }
    }
    /// Not done! Please wait for this to be finished or open a PR!
    #[cfg(feature = "encryption")]
    pub fn from_reader_enc_com<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
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
        num_traits::FromPrimitive::from_u8(value)
            .ok_or(Error::EnumOutOfBound)
    }
}

impl TryFrom<VarInt> for ChatSettings {
    type Error = Error;
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_i32(value.value())
            .ok_or(Error::EnumOutOfBound)
    }
}

impl Into<VarInt> for ChatSettings {
    fn into(self) -> VarInt {
        // This is a safe unwrap: no enum value exceeds safe VarInt limits.
        VarInt::from_value(self as i32).unwrap()
    }
}

impl ChatSettings {
    pub fn to_varint(self) -> VarInt {
        self.into()
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct SkinSettings: u8 {
        const CAPE =        1 << 0;
        const JACKET =      1 << 1;
        const LEFT_SLEVE =  1 << 2;
        const RIGHT_SLEVE = 1 << 3;
        const LEFT_LEG =    1 << 4;
        const RIGHT_LEG =   1 << 5;
        const HAT =         1 << 6;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String
}
