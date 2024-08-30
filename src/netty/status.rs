use crate::{Error, VarInt};
use crate::generalized::{long_from_reader, long_to_bytes, string_from_reader, string_to_bytes};
use std::io::Read;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ServerboundPacket {
    StatusRequest,
    PingRequest {
        payload: i64
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ClientboundPacket {
    StatusResponse {
        // TODO: https://wiki.vg/Server_List_Ping#Status_Response
        response: String
    },
    PingResponse {
        payload: i64
    }
}

impl ServerboundPacket {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        match self {
            Self::StatusRequest => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x00)?.to_bytes()?);
            }
            Self::PingRequest { payload } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x01)?.to_bytes()?);
                // Payload
                bytes.append(&mut long_to_bytes(*payload)?);
            }
        }
        // Calculate packet length, prepend, and send it!
        let packet_length = bytes.len();
        let mut result = VarInt::from_value(packet_length as i32)?.to_bytes()?;
        result.append(&mut bytes);
        return Ok(result);
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let _packet_length = VarInt::from_reader(reader)?;
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00 => return Ok(ServerboundPacket::StatusRequest),
            0x01 => {
                let payload = long_from_reader(reader)?;
                return Ok(ServerboundPacket::PingRequest { payload });
            }
            _ => {
                return Err(Error::InvalidPacketID);
            }
        }
    }
}

impl ClientboundPacket {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        match self {
            Self::StatusResponse { response } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x00)?.to_bytes()?);
                // Payload
                bytes.append(&mut string_to_bytes(response.clone())?);
            }
            Self::PingResponse { payload } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x01)?.to_bytes()?);
                // Payload
                bytes.append(&mut long_to_bytes(*payload)?);
            }
        }
        // Calculate packet length, prepend, and send it!
        let packet_length = bytes.len();
        let mut result = VarInt::from_value(packet_length as i32)?.to_bytes()?;
        result.append(&mut bytes);
        return Ok(result);
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let _packet_length = VarInt::from_reader(reader)?;
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00 => {
                let response = string_from_reader(reader)?;
                return Ok(ClientboundPacket::StatusResponse { response });
            }
            0x01 => {
                let payload = long_from_reader(reader)?;
                return Ok(ClientboundPacket::PingResponse { payload });
            }
            _ => {
                return Err(Error::InvalidPacketID);
            }
        }
    }
}
