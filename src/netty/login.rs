use crate::{Error, Identifier, VarInt, UUID};
use crate::generalized::{boolean_from_reader, string_from_reader_no_cesu8, string_to_bytes_no_cesu8};
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
    EncryptionRequest {
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
                bytes.append(&mut string_to_bytes_no_cesu8(name.clone())?);
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
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn to_bytes_com(&self, threshold: VarInt) -> Result<Vec<u8>, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn to_bytes_enc(&self) -> Result<Vec<u8>, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn to_bytes_enc_com(&self, threshold: VarInt) -> Result<Vec<u8>, Error> {
        todo!()
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
                let bool_result = boolean_from_reader(reader)?;
                if bool_result {
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
                    return Ok(ServerboundPacket::LoginPluginResponse {
                        message_id,
                        data: None
                    });
                }
            }
            0x03 => return Ok(ServerboundPacket::LoginAcknowledged),
            0x04 => {
                let key = Identifier::from_reader(reader)?;
                let bool_result = boolean_from_reader(reader)?;
                if bool_result {
                    let dta_len = VarInt::from_reader(reader)?;
                    let mut data = vec![0; dta_len.value() as usize];
                    reader.read_exact(&mut data).unwrap();
                    return Ok(ServerboundPacket::CookieResponse {
                        key,
                        payload: Some(data)
                    });
                    
                }
                else {
                    return Ok(ServerboundPacket::CookieResponse {
                        key,
                        payload: None
                    });
                }
            },
            _ => {
                return Err(Error::InvalidPacketId);
            }
        }
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn from_reader_enc<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn from_reader_com<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn from_reader_enc_com<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
}

impl ClientboundPacket {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        match self {
            Self::Disconnect { reason } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x00)?.to_bytes()?);

                // Payload
                // TODO: this may need cesu8 conversion?
                bytes.append(&mut string_to_bytes_no_cesu8(reason.clone())?);
            }
            Self::EncryptionRequest {
                server_id, public_key, verify_token,
                should_authenticate
            } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x01)?.to_bytes()?);

                // Payload
                // Server ID
                assert!(server_id.chars().count() <= 20);
                bytes.append(&mut string_to_bytes_no_cesu8(server_id.clone())?);
                // Public Key
                bytes.append(&mut VarInt::from_value(public_key.len() as i32)?.to_bytes()?);
                bytes.append(&mut public_key.clone());
                // Verify Token
                bytes.append(&mut VarInt::from_value(verify_token.len() as i32)?.to_bytes()?);
                bytes.append(&mut verify_token.clone());
                // Should Authenticate
                bytes.push(if *should_authenticate { 0x01 } else { 0x00 });
            }
            Self::LoginSuccess {
                uuid, username, properties,
                strict_error_handling
            } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x02)?.to_bytes()?);

                // Payload
                // UUID
                bytes.append(&mut uuid.to_bytes()?);
                // Username
                assert!(username.chars().count() <= 16);
                bytes.append(&mut string_to_bytes_no_cesu8(username.clone())?);

                // Properties len
                bytes.append(&mut VarInt::from_value(properties.len() as i32)?.to_bytes()?);
                // Properties
                for property in properties {
                    assert!(property.name.chars().count() <= 32767);
                    bytes.append(&mut string_to_bytes_no_cesu8(property.name.clone())?);
                    assert!(property.value.chars().count() <= 32767);
                    bytes.append(&mut string_to_bytes_no_cesu8(property.value.clone())?);
                    if let Some(signature) = &property.signature {
                        bytes.push(0x01);
                        assert!(signature.chars().count() <= 32767);
                        bytes.append(&mut string_to_bytes_no_cesu8(signature.clone())?);
                    }
                    else {
                        bytes.push(0x00);
                    }
                }

                // Error Handling
                bytes.push(if *strict_error_handling { 0x01 } else { 0x00 });
            }
            Self::SetCompression { threshold } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x03)?.to_bytes()?);

                // Payload
                bytes.append(&mut threshold.to_bytes()?);
            }
            Self::LoginPluginRequest {
                message_id, channel, data
            } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x04)?.to_bytes()?);

                // Payload
                // Message ID
                bytes.append(&mut message_id.to_bytes()?);
                // Channel
                bytes.append(&mut channel.to_bytes()?);
                // Data
                // TODO: this clone is gross. Something must be done!
                assert!(data.len() <= 1048576);
                bytes.append(&mut data.clone());
            }
            Self::CookieRequest { key } => {
                // Packet ID
                bytes.append(&mut VarInt::from_value(0x05)?.to_bytes()?);

                // Payload
                bytes.append(&mut key.to_bytes()?);
            }
        }
        // Calculate packet length, prepend, and send it!
        let packet_length = bytes.len();
        let mut result = VarInt::from_value(packet_length as i32)?.to_bytes()?;
        result.append(&mut bytes);
        return Ok(result);
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn to_bytes_com(&self, threshold: VarInt) -> Result<Vec<u8>, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn to_bytes_enc(&self) -> Result<Vec<u8>, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn to_bytes_enc_com(&self, threshold: VarInt) -> Result<Vec<u8>, Error> {
        todo!()
    }
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let packet_length = VarInt::from_reader(reader)?;
        let packet_id = VarInt::from_reader(reader)?;
        match packet_id.value() {
            0x00 => {
                let reason = string_from_reader_no_cesu8(reader)?;
                
                return Ok(Self::Disconnect { reason });
            }
            0x01 => {
                let server_id = string_from_reader_no_cesu8(reader)?;

                let public_key_len = VarInt::from_reader(reader)?;
                let mut public_key = vec![0x00; public_key_len.value() as usize];
                reader.read_exact(&mut public_key)?;

                let verify_token_len = VarInt::from_reader(reader)?;
                let mut verify_token = vec![0x00; verify_token_len.value() as usize];
                reader.read_exact(&mut verify_token)?;

                let should_authenticate = boolean_from_reader(reader)?;
                
                return Ok(Self::EncryptionRequest {
                    server_id, public_key, verify_token, should_authenticate
                });
            }
            0x02 => {
                let uuid = UUID::from_reader(reader)?;
                let username = string_from_reader_no_cesu8(reader)?;

                let properties_len = VarInt::from_reader(reader)?.value();
                let mut properties = vec![];

                for _ in 0..properties_len {
                    let name = string_from_reader_no_cesu8(reader)?;
                    let value = string_from_reader_no_cesu8(reader)?;
                    let is_signed = boolean_from_reader(reader)?;
                    let signature = if is_signed {
                        Some(string_from_reader_no_cesu8(reader)?) 
                    } else { None };
                    let property = Property { name, value, signature };
                    properties.push(property);
                }

                let strict_error_handling = boolean_from_reader(reader)?;

                return Ok(Self::LoginSuccess {
                    uuid, username, properties, strict_error_handling
                });
            }
            0x03 => {
                let threshold = VarInt::from_reader(reader)?;

                return Ok(Self::SetCompression { threshold });
            }
            0x04 => {
                let message_id = VarInt::from_reader(reader)?;
                let channel = Identifier::from_reader(reader)?;
                // These unwraps are safe: we just pulled this data and know it
                // must have a read size value!
                let data_len = 
                    packet_length.value() as usize -
                    packet_id.read_size().unwrap() as usize -
                    message_id.read_size().unwrap() as usize -
                    channel.to_bytes()?.len();
                
                let mut data = vec![0x00; data_len];

                reader.read_exact(&mut data)?;

                return Ok(Self::LoginPluginRequest { message_id, channel, data });
            }
            0x05 => {
                let key = Identifier::from_reader(reader)?;

                return Ok(Self::CookieRequest { key });
            },
            _ => {
                return Err(Error::InvalidPacketId);
            }
        }
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn from_reader_enc<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn from_reader_com<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
    /// Not done! Please wait for this to be finished or open a PR!
    pub fn from_reader_enc_com<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
}
