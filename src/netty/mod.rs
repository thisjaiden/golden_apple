/// Enums and packets for communicating with traditional Minecraft software
/// during the inital "handshake" stage of a connection. Note that there are no
/// clientbound packets during this phase, and that the only serverbound packet
/// immediately changes both the client and server's stage to a different one.
pub mod handshake;
mod status;

pub enum ServerboundPacket {
    Handshake(handshake::ServerboundPacket),
    Status(status::ServerboundPacket),
}

pub enum ClientboundPacket {
    Status(status::ClientboundPacket),
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
        return num_traits::FromPrimitive::from_u8(value).ok_or(Self::Error::EnumOutOfBound);
    }
}
