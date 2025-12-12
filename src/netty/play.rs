use std::io::Read;
use crate::Error;

// TODO
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ClientboundPacket {

}

impl ClientboundPacket {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        todo!()
    }
}