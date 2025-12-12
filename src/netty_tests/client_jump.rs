use std::io::Write;
use crate::{PROTOCOL_VERSION, VarInt, netty};

const USERNAME: &'static str = "not_a_real_username";

#[test]
#[ignore = "There's a number of reasons to run this test sparingly. See the source for more info."]
/// This test connects to a server, jumps, and disconnects. This test is not and
/// will never be marked to be run automatically. Connecting over the internet
/// is not a mature choice in most CI environments, and this test requires a
/// recepient server which has to take a strange client connecting. Hardcoding
/// an arbitrary server and authentication is also not a reliable strategy.
fn client_jump() {
    // Connect to the remote server
    let mut connection = std::net::TcpStream::connect("play.wynncraft.com:25565")
        .expect("Unable to connect to the remote server.");
    // Request to connect to the server
    connection.write_all(
        &netty::handshake::ServerboundPacket::Handshake {
            protocol_version: VarInt::from_value(PROTOCOL_VERSION).unwrap(),
            server_address: String::from("104.17.17.42"),
            server_port: 25565,
            next_state: netty::handshake::NextState::Login
        }.to_bytes().unwrap()
    ).unwrap();
    connection.flush().unwrap();

    // We are now in the "login" stage!
    connection.write_all(
        &netty::login::ServerboundPacket::LoginStart {
            name: String::from(USERNAME),
            uuid: crate::UUID::from_username(USERNAME).unwrap()
        }.to_bytes().unwrap()
    ).unwrap();
    // More work to be done, this is where encryption is enabled and auth is
    // verified. This test is incomplete for now.
    todo!();
}