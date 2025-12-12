use std::io::Write;
use crate::{PROTOCOL_VERSION, VarInt, netty};

#[test]
#[ignore = "There's a number of reasons to run this test sparingly. See the source for more info."]
/// This test connects to a server, gets the status and MOTD, and disconnects.
/// This test is not and will never be marked to be run automatically.
/// Connecting over the internet is not a mature choice in most CI environments,
/// and this test requires a recepient server which has to take a strange client
/// connecting. Hardcoding an arbitrary server and authentication is also not a
/// reliable strategy.
fn client_ping() {
    // Connect to the remote server
    let mut connection = std::net::TcpStream::connect("play.wynncraft.com:25565")
       .expect("Unable to connect to the remote server.");
    // Request to connect to the server
    connection.write_all(
        &netty::handshake::ServerboundPacket::Handshake {
            protocol_version: VarInt::from_value(PROTOCOL_VERSION).unwrap(),
            server_address: String::from("play.wynncraft.com"),
            server_port: 25565,
            next_state: netty::handshake::NextState::Status
        }.to_bytes().unwrap()
    ).unwrap();
    // Make sure that the packet is fully sent
    connection.flush().unwrap();

    // The server should now be in the status state. Let's test that by sending
    // a status request!
    connection.write_all(
        &netty::status::ServerboundPacket::StatusRequest.to_bytes().unwrap()
    ).unwrap();
    // And like before, make sure we flush our buffer :) If you're smart you
    // could let both packets get sent together and flush at this point (the
    // first time you might get something back)
    connection.flush().unwrap();

    // Let's see what the server has to say.
    let result = netty::status::ClientboundPacket::from_reader(&mut connection).unwrap();
    // And let's make sure this part isn't optimized out, as we never actually
    // look at what the server says.
    std::hint::black_box(result);

    // Assuming we could parse the packet from the server, this test has
    // succeeded. Let's clean up a bit!
    connection.shutdown(std::net::Shutdown::Both).unwrap()
}