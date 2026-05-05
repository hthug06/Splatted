use crate::network::connection::Encryption;
use crate::packets::ClientPacket;
use crate::packets::ServerPacket;
use crate::packets::packet2_client_protocol::ClientProtocol;
use crate::packets::packet205_client_command::ClientCommandPacket;
use crate::packets::packet252_shared_key::SharedKeyPacket;
use crate::packets::packet253_server_auth_data::ServerAuthData;
use std::io::{Cursor, Error, ErrorKind};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedWriteHalf;

pub struct Client {
    username: String,
    /// Here, we only take a writer because the reader will always be active in the connect function.
    /// Also, we use an option because we're only going to connect the client in the connect function.
    writer: Option<OwnedWriteHalf>,
    /// The connection contain the encryption process.
    /// (only active after packet 252)
    encryption: Encryption,
}

impl Client {
    pub fn new(username: &str) -> Client {
        Self {
            username: username.to_string(),
            writer: None,
            encryption: Encryption::new(),
        }
    }

    pub async fn connect(&mut self, address: &str) -> std::io::Result<()> {
        let stream = TcpStream::connect(address).await?;
        stream.set_nodelay(true)?;

        let (mut reader, writer) = stream.into_split();

        let socket_addr: SocketAddr = address
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let host = socket_addr.ip().to_string();
        let port = socket_addr.port() as u32;

        // Init writer
        self.writer = Some(writer);

        //After this, we can send the first packet
        // this packet contain the protocol version of the client (51 in 1.4.7)
        // the username, the address and port of the server
        let handshake = ClientProtocol::new(51, &self.username, host, port);
        self.send_packet(handshake).await?;

        // Create a buffer
        // in this version, we can't know before the size of the buffer
        // 4096 might be good for now, but we might need to extend it when parsing chunk packet...
        // /** A temporary storage for the compressed chunk data byte array. */
        // private static byte[] temp = new byte[196864]; in source code so yeeeee...
        let mut buffer: [u8; 4096] = [0; 4096];

        // Listen for packet
        loop {
            let bytes_read: usize = reader.read(&mut buffer).await?;

            if bytes_read == 0 {
                log::info!("Connection closed by server");
                break;
            }

            // The raw data received from the server
            let received_data: &mut [u8] = &mut buffer[..bytes_read];

            // decrypt the connection if needed
            self.encryption.decrypt(received_data);

            log::info!("Received data: {:?}", received_data);

            // The packet id is always the first byte
            let packet_id: u8 = received_data[0];

            match packet_id {
                255 => {
                    // Server closed the connection (kick or normal disconnect)
                    break;
                }
                253 => {
                    self.handle_server_auth_data(&received_data[1..]).await?;
                }
                252 => {
                    self.handle_shared_key(&received_data[1..]).await?;
                }
                1 => {
                    log::info!("Login packet received");
                }
                id => log::warn!("Packet {} unknown", id),
            };
        }

        Ok(())
    }

    /// Send a packet to the network instantly
    pub async fn send_packet(&mut self, packet: impl ClientPacket) -> std::io::Result<()> {
        let mut buffer: Vec<u8> = Vec::new();

        packet
            .write_to(&mut buffer)
            .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;

        log::info!("Sending packet: {:?}", &buffer);

        // Fill the buffer with the packet data
        // Encrypt the packet if the encryption is enabled
        self.encryption.encrypt(&mut buffer);

        if let Some(writer) = &mut self.writer {
            writer.write_all(&buffer).await?;
            writer.flush().await?;
        } else {
            log::error!("Trying to send a packet without being connected");
        }

        Ok(())
    }

    /// Handle the packet 253 (0xFD)
    /// First, get the data from the packet (token and public key).
    /// Then, create the Shared key packet (252), with the shared secret between the client and the server.
    /// Finally, send the Shared key packet and prepare the cipher (activated on 0xFC confirmation).
    pub async fn handle_server_auth_data(&mut self, data: &[u8]) -> std::io::Result<()> {
        let packet: ServerAuthData = match ServerAuthData::read(&mut Cursor::new(data)) {
            Ok(packet) => packet,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
        };
        log::info!("AuthData (0xFD) received");

        if packet.server_id != "-" {
            return Err(Error::new(
                ErrorKind::Unsupported,
                "Server is in online mode",
            ));
        }

        // We fully handle the packet, we can now create the packet 252 and send it
        let (packet_252, shared_secret) =
            SharedKeyPacket::new(&packet.verify_token, &packet.public_key);

        // After this packet, every sent packet will be encrypted
        self.send_packet(packet_252).await?;

        // Create the shared secret to start the encryption
        let secret: [u8; 16] = shared_secret
            .as_slice()
            .try_into()
            .expect("shared_secret must be exactly 16 bytes for AES-128");

        log::info!("shared_secret ({} bytes): {:?}", secret.len(), secret);

        self.encryption.set_encryption(&secret);

        Ok(())
    }

    /// Handle the packet 252 (0xFC)
    /// First, parse the received packet.
    /// Then, if the packet is right, confirm the encryption.
    /// Finally, send the ClientCommandPacket (205) to spawn the client in the world.
    pub async fn handle_shared_key(&mut self, data: &[u8]) -> std::io::Result<()> {
        // Packet to confirm if the server confirm
        let packet = match SharedKeyPacket::read(&mut Cursor::new(data)) {
            Ok(packet) => packet,
            Err(e) => panic!("Failed to read server shared key packet: {}", e),
        };
        log::info!("Shared Key Packet received");

        // From now, every sent and received packet will be encrypted
        if packet.is_encryption_confirmed() {
            self.encryption.enable_encryption()
        } else {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid shared key, bad encryption",
            ));
        }
        self.send_packet(ClientCommandPacket::new(0)).await?;

        Ok(())
    }
}
