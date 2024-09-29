use crate::{Chat, Error, VarInt, UUID};
use crate::generalized::{
    long_from_reader, long_to_bytes, string_from_reader_no_cesu8,
    string_to_bytes_no_cesu8, string_to_writer_no_cesu8
};
use std::io::Read;

#[derive(Clone, PartialEq, Eq, Debug)]
/// A packet sent from the client to the server during the "status" phase.
/// 
/// Note that the Notchian server has some weirdness around packet order. See
/// [wiki.vg](https://wiki.vg/Protocol#Status) for more info.
pub enum ServerboundPacket {
    StatusRequest,
    PingRequest {
        payload: i64
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// A packet sent from the server to the client during the "status" phase.
pub enum ClientboundPacket {
    StatusResponse {
        response: StatusResponse
    },
    PingResponse {
        payload: i64
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// The information returned from a server when querying a server's status.
pub struct StatusResponse {
    pub version_name: String,
    pub version_protocol: i64,
    pub max_players: i64,
    pub online_players: i64,
    pub favicon_data: String,
    pub sample_players: Vec<(String, UUID)>,
    pub description: Chat
}

impl StatusResponse {
    // TODO: do this the proper way and not with this crud...
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<StatusResponse, Error> {
        let raw_data = string_from_reader_no_cesu8(reader)?;
        let json_data: serde_json::Value = serde_json::from_str(&raw_data)?;

        Ok(StatusResponse {
            version_name: json_data["version"]["name"].to_string(),
            version_protocol: json_data["version"]["protocol"].as_i64().ok_or(Error::InvalidJsonRoot)?,
            max_players: json_data["players"]["max"].as_i64().ok_or(Error::InvalidJsonRoot)?,
            online_players: json_data["players"]["online"].as_i64().ok_or(Error::InvalidJsonRoot)?,
            description: Chat::from_string(serde_json::to_string(&json_data["description"])?)?,
            favicon_data:
                json_data["favicon"]
                    .as_str()
                    .ok_or(Error::InvalidJsonRoot)?
                    .to_string()
                    .trim_start_matches("data:image/png;base64,")
                    .to_string(),
            sample_players:
                json_data["players"]["sample"]
                    .as_array()
                    .ok_or(Error::InvalidJsonRoot)
                    .map(|dta| {
                        let mut final_data = vec![];
                        for pair in dta {
                            final_data.push((pair["name"].to_string(), UUID::from_username(pair["id"].to_string()).unwrap()));
                        }

                        final_data
                    })?
        })
    }
    fn to_string(&self) -> Result<String, Error> {
        let mut string_data = String::new();
        string_data += "{\"version\":{\"name\":\"";
        string_data += &self.version_name;
        string_data += "\",\"protocol\":";
        string_data += &format!("{}", self.version_protocol);
        string_data += "},\"players\":{\"max\":";
        string_data += &format!("{}", self.max_players);
        string_data += ",\"online\":";
        string_data += &format!("{}", self.online_players);
        string_data += "\"sample\":[";
        let mut sample_index = false;
        for player in self.sample_players.clone() {
            if sample_index {
                string_data += ",";
            }
            string_data += "{\"name\":\"";
            string_data += &player.0;
            string_data += "\",\"id\":\"";
            string_data += &format!("{:x}", player.1.to_value()?);
            string_data += "}";
            sample_index = true;
        }
        string_data += "]},\"description\":";
        string_data += ",\"favicon\":\"data:image/png;base64,";
        string_data += &self.favicon_data;
        string_data += "\"}";

        Ok(string_data)
    }

    pub fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), Error> {
        string_to_writer_no_cesu8(writer, self.to_string()?)?;

        Ok(())
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        string_to_bytes_no_cesu8(self.to_string()?)
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

        Ok(result)
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let _packet_length = VarInt::from_reader(reader)?;
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00 => Ok(ServerboundPacket::StatusRequest),
            0x01 => {
                let payload = long_from_reader(reader)?;

                Ok(ServerboundPacket::PingRequest { payload })
            }
            _ => Err(Error::InvalidPacketId(packet_id))
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
                bytes.append(&mut response.to_bytes()?);
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

        Ok(result)
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let _packet_length = VarInt::from_reader(reader)?;
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00 => {
                let response = StatusResponse::from_reader(reader)?;

                Ok(ClientboundPacket::StatusResponse { response })
            }
            0x01 => {
                let payload = long_from_reader(reader)?;

                Ok(ClientboundPacket::PingResponse { payload })
            }
            _ => Err(Error::InvalidPacketId(packet_id))
        }
    }
}
