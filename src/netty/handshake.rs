use crate::{generalized::unsigned_short_to_bytes, Error, VarInt};
use std::io::Read;
use crate::generalized::{string_from_reader, unsigned_short_from_reader, string_to_bytes_no_cesu8};

#[derive(Clone, PartialEq, Eq, Debug)]
/// A packet sent from the client to the server during the "handshake" phase.
pub enum ServerboundPacket {
    /// Indicates to the server what this connection is for.
    Handshake {
        /// The version of the netty protocol being used ([crate::PROTOCOL_VERSION])
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        /// Which network state both the client and server should switch to
        /// after this packet.
        next_state: NextState,
    }
}

impl ServerboundPacket {
    /// Converts this packet into bytes that can be sent over the network to a
    /// server using this protocol version.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        match self {
            Self::Handshake {
                protocol_version, server_address,
                server_port, next_state
            } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0)?.to_bytes()?);
                // Fields
                bytes.append(&mut protocol_version.to_bytes()?);
                bytes.append(&mut string_to_bytes_no_cesu8(server_address.clone())?);
                bytes.append(&mut unsigned_short_to_bytes(*server_port)?);
                let tryinto: VarInt = VarInt::try_from(*next_state)?;
                bytes.append(&mut tryinto.to_bytes()?);
            }
        }
        let packet_length = bytes.len();
        let mut result = VarInt::from_value(packet_length as i32)?.to_bytes()?;
        result.append(&mut bytes);
        return Ok(result);
    }
    /// Deserializes a packet from a [Read] type
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let _packet_length = VarInt::from_reader(reader)?;
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00 => {
                let protocol_version = VarInt::from_reader(reader)?;
                let server_address = string_from_reader(reader)?;
                let server_port = unsigned_short_from_reader(reader)?;
                let next_state = NextState::try_from(VarInt::from_reader(reader)?)?;
                return Ok(ServerboundPacket::Handshake {
                    protocol_version, server_address, server_port, next_state
                });
            }
            _ => {
                return Err(Error::InvalidPacketId);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, ToPrimitive, FromPrimitive)]
/// Indicates what state to switch the protocol between client and server to.
pub enum NextState {
    /// Switch to the Status state. (used for the server list)
    Status = 1,
    /// Switch to the Login state.
    Login = 2,
    /// Similar to Login, but indicates a transfer of server
    Transfer = 3,
}

impl TryFrom<VarInt> for NextState {
    type Error = Error;
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        return num_traits::FromPrimitive::from_i32(value.value()).ok_or(Error::EnumOutOfBound);
    }
}

impl TryFrom<NextState> for VarInt {
    type Error = Error;
    fn try_from(value: NextState) -> Result<crate::VarInt, Self::Error> {
        return VarInt::from_value(value as i32);
    }
}
