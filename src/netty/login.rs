use crate::{Error, Identifier, VarInt, UUID};
use crate::generalized::{unsigned_byte_from_reader, string_from_reader_no_cesu8, string_to_bytes_no_cesu8};
use std::io::Read;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ServerboundPacket {
    LoginStart {
        name: String,
        uuid: UUID
    },
    /// All packets after EncryptionResponse should be encrypted. If
    /// authentication is enabled, this is when the server authenticates.
    EncryptionResponse {
        shared_secret: Vec<u8>,
        verify_token: Vec<u8>
    },
    LoginPluginResponse {
        message_id: VarInt,
        data: Option<Vec<u8>>
    },
    LoginAcknowledged,
    CookieResponse {
        key: Identifier,
        payload: Option<Vec<u8>>,
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ClientboundPacket {
    Disconnect {
        reason: String // TODO: https://wiki.vg/Protocol#Type:JSON_Text_Component
    },
    Login {
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>,
        should_authenticate: bool
    },
    LoginSuccess {
        uuid: UUID,
        username: String,
        properties: Vec<Property>,
        strict_error_handling: bool
    },
    SetCompression {
        threshold: VarInt
    },
    LoginPluginRequest {
        message_id: VarInt,
        channel: Identifier,
        data: Vec<u8>
    },
    CookieRequest {
        key: Identifier
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Property {
    name: String,
    value: String,
    signature: Option<String>
}


impl ServerboundPacket {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        match self {
            Self::LoginStart { name, uuid } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x00)?.to_bytes()?);

                // Payload (username, UUID)
                // Anything larger than 16 characters is invalid.
                assert!(name.chars().count() <= 16);
                bytes.append(&mut string_to_bytes_no_cesu8(name.to_string())?);
                bytes.append(&mut uuid.to_bytes()?);
            }
            Self::EncryptionResponse { shared_secret, verify_token } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x01)?.to_bytes()?);
                
                // Payload
                // Shared Secret Length
                bytes.append(&mut VarInt::from_value(shared_secret.len() as i32)?.to_bytes()?);
                // Shared Secret
                bytes.append(&mut shared_secret.clone());
                // Verify Token Length
                bytes.append(&mut VarInt::from_value(verify_token.len() as i32)?.to_bytes()?);
                // Verify Token
                bytes.append(&mut verify_token.clone());
            }
            Self::LoginPluginResponse { message_id, data: data_opt } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x02)?.to_bytes()?);

                // Payload
                // Message ID
                bytes.append(&mut message_id.to_bytes()?);
                // Successful
                if let Some(data) = data_opt {
                    bytes.push(0x01);
                    // The Notchian server will choke on anything larger.
                    assert!(data.len() <= 1048576);
                    bytes.append(&mut data.clone());
                }
                else {
                    bytes.push(0x00);
                }
            }
            Self::LoginAcknowledged => {
                // Packet ID, that's it!
                bytes.append(&mut VarInt::from_value(0x03)?.to_bytes()?);
            }
            Self::CookieResponse { key, payload } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x04)?.to_bytes()?);

                // Payload
                // Identifier
                bytes.append(&mut key.clone().to_bytes()?);
                if let Some(payload) = payload {
                    // Cookies must be 5kib or less
                    assert!(payload.len() <= 5120);
                    // Has payload
                    bytes.push(0x01);
                    // Payload len
                    bytes.append(&mut VarInt::from_value(payload.len() as i32)?.to_bytes()?);
                    // Payload
                    bytes.append(&mut payload.clone())
                }
                else {
                    // No payload
                    bytes.push(0x00);
                }
            }
        }
        // Calculate packet length, prepend, and send it!
        let packet_length = bytes.len();
        let mut result = VarInt::from_value(packet_length as i32)?.to_bytes()?;
        result.append(&mut bytes);
        return Ok(result);
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let packet_length = VarInt::from_reader(reader)?;
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00 => {
                let name = string_from_reader_no_cesu8(reader)?;
                let uuid = UUID::from_reader(reader)?;

                return Ok(ServerboundPacket::LoginStart { name, uuid })
            }
            0x01 => {
                let array_length = VarInt::from_reader(reader)?;
                let mut shared_secret = vec![0; array_length.value() as usize];
                reader.read_exact(&mut shared_secret).unwrap();
                let array_length = VarInt::from_reader(reader)?;
                let mut verify_token = vec![0; array_length.value() as usize];
                reader.read_exact(&mut verify_token).unwrap();

                return Ok(ServerboundPacket::EncryptionResponse {
                    shared_secret, verify_token
                });
            }
            0x02 => {
                let message_id = VarInt::from_reader(reader)?;
                let bool_result = unsigned_byte_from_reader(reader)?;
                if bool_result == 0x00 {
                    return Ok(ServerboundPacket::LoginPluginResponse {
                        message_id,
                        data: None
                    });
                }
                else if bool_result == 0x01 {
                    let dta_len =
                        packet_length.value() as usize -
                        packet_id.read_size().unwrap() as usize -
                        message_id.read_size().unwrap() as usize -
                        1;
                    let mut data = vec![0; dta_len];
                    reader.read_exact(&mut data).unwrap();
                    return Ok(ServerboundPacket::LoginPluginResponse {
                        message_id,
                        data: Some(data)
                    });
                }
                else {
                    // TODO: proper error instead of panic
                    panic!("Invalid bool!")
                }
            }
            0x03..0x04 => todo!(),
            _ => {
                return Err(Error::InvalidPacketId);
            }
        }
    }
}
