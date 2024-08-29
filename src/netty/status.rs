use crate::{VarInt, Error};

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
