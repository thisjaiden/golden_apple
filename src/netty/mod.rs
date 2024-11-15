/// Enums and packets for communicating with traditional Minecraft software
/// during the inital "handshake" stage of a connection.
/// 
/// Note that there are no clientbound packets during this phase, and that the
/// only serverbound packet immediately changes both the client and server's
/// stage to a different one.
pub mod handshake;

/// Packets and structs for communicating with traditional Minecraft software
/// during the "status" stage of a connection.
/// 
/// Note that this is a connection dead-end, and some conditions apply to the
/// order in which packets should be sent and recieved. For more information,
/// see [wiki.vg](https://wiki.vg/Protocol#Status).
pub mod status;

/// Structs and packets for communicating with traditional Minecraft software
/// during the "login" stage of a connection.
/// 
/// This is the stage at which compression and encryption may be enabled, so all
/// conversion tools will have the following variants:
/// - `_com` Compressed
/// - `_enc` Encrypted
/// - `_enc_com` Encrypted & Compressed  
/// 
/// The variants including encryption require the `encryption` cargo feature.
pub mod login;

/// Structs, packets, and enums for communicating with traditional Minecraft
/// sofrtware during the "configuration" stage of a connection.
pub mod configuration;


/// Represents all the packets that may be sent to the server at various stages
/// of a client-server interaction.
pub enum ServerboundPacket {
    /// Serverbound packets immediately following the start of a connection.
    /// Known as the "handshake" stage.
    Handshake(handshake::ServerboundPacket),
    /// Serverbound packets if a client requests to switch to the "status" stage.
    Status(status::ServerboundPacket),
    /// Serverbound packets if a client requests to switch to the "login" stage.
    Login(login::ServerboundPacket),
}

/// Represents all the packets that may be sent to the client at various stages
/// of a client-server interaction.
pub enum ClientboundPacket {
    Status(status::ClientboundPacket),
    Login(login::ClientboundPacket),
}

impl ClientboundPacket {
    pub fn from_reader<R: std::io::Read>(
        reader: &mut R, protocol_state: ProtocolState
    ) -> Result<Self, crate::Error> {
        match protocol_state {
            ProtocolState::Handshake => {
                Err(crate::Error::NoClientboundHandshake)
            },
            ProtocolState::Status => {
                Ok(ClientboundPacket::Status(
                    status::ClientboundPacket::from_reader(reader)?
                ))
            },
            ProtocolState::Login => {
                Ok(ClientboundPacket::Login(
                    login::ClientboundPacket::from_reader(reader)?
                ))
            }
            _ => todo!()
        }
    }
    pub fn from_reader_com<R: std::io::Read>(
        reader: &mut R, protocol_state: ProtocolState
    ) -> Result<Self, crate::Error> {
        match protocol_state {
            ProtocolState::Handshake | ProtocolState::Status => {
                panic!("It's not possible for packets to be compressed during these stages of networking!");
            },
            ProtocolState::Login => {
                Ok(ClientboundPacket::Login(
                    login::ClientboundPacket::from_reader_com(reader)?
                ))
            }
            _ => todo!()
        }
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
    /// The Configuration state is for setting resource packs, plugins, and cookies.
    Configuration = 3,
    /// The Play state is for standard gameplay.
    Play = 4
}

impl TryFrom<u8> for ProtocolState {
    type Error = crate::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_u8(value)
            .ok_or(Self::Error::EnumOutOfBound)
    }
}
