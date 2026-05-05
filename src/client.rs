use crate::packets::ClientPacket;
use crate::packets::ServerPacket;
use crate::packets::packet2_client_protocol::ClientProtocol;
use crate::packets::packet205_client_command::ClientCommandPacket;
use crate::packets::packet252_shared_key::SharedKeyPacket;
use crate::packets::packet253_server_auth_data::ServerAuthData;
use aes::Aes128;
use cfb8::{Decryptor, Encryptor};
use cipher::KeyIvInit;
use log::info;
use std::cmp::PartialEq;
use std::io::{Cursor, Error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedWriteHalf;

type AesCfb8Enc = Encryptor<Aes128>;
type AesCfb8Dec = Decryptor<Aes128>;

#[derive(PartialEq)]
enum ConnectionState {
    HandShake,
    Encrypted,
}

pub struct Client {
    username: String,
    /// Here, we only take a writer because the reader will always be active in the connect function.
    /// Also, we use an option because we're only going to connect the client in the connect function.
    writer: Option<OwnedWriteHalf>,
    encryptor: Option<AesCfb8Enc>,
    decryptor: Option<AesCfb8Dec>,
    connection_state: ConnectionState,
}

impl Client {
    pub fn new(username: &str) -> Client {
        Self {
            username: username.to_string(),
            writer: None,
            encryptor: None,
            decryptor: None,
            connection_state: ConnectionState::HandShake,
        }
    }

    pub async fn connect(&mut self, address: &str) -> std::io::Result<()> {
        let stream = TcpStream::connect(address).await?;
        stream.set_nodelay(true)?;

        let (mut reader, writer) = stream.into_split();

        let parts: Vec<&str> = address.split(':').collect();
        let host: String = parts[0].to_string();
        let port: u32 = parts.get(1).unwrap_or(&"25565").parse::<u32>().unwrap();

        // Init writer
        self.writer = Some(writer);

        //After this, we can send the first packet
        let handshake = ClientProtocol::new(51, &self.username, host, port);
        self.send_packet(handshake).await?;

        //Create a buffer
        // in this version, we can't know before the size of the buffer
        // 1024 might be good for now, but we might need to extend it when parsing chunk packet...
        // /** A temporary storage for the compressed chunk data byte array. */
        // private static byte[] temp = new byte[196864]; in source code so yeeeee...
        let mut buffer: [u8; 4096] = [0; 4096];

        // Listen for packet
        loop {
            let bytes_read: usize = reader.read(&mut buffer).await?;

            if bytes_read == 0 {
                continue;
            }

            let received_data: &mut [u8] = &mut buffer[..bytes_read];

            // encrypt when the decryptor is ready AND only after the packet 252 is received
            if self.connection_state == ConnectionState::Encrypted {
                if let Some(dec) = &mut self.decryptor {
                    dec.decrypt(received_data);
                    log::info!("decrypted {} bytes", bytes_read);
                }
            }
            // The packet id is always the first byte
            let packet_id: u8 = received_data[0];

            log::info!("Received data (decrypted or not): {:?}", received_data);

            match packet_id {
                255 => {
                    // 255 is the kick / disconnect packet, so we stop the listener
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
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        log::info!("Sending packet: {:?}", &buffer);

        if let Some(enc) = &mut self.encryptor {
            enc.encrypt(&mut buffer);
        }

        if let Some(writer) = &mut self.writer {
            writer.write_all(&buffer).await?;
            writer.flush().await?;
        } else {
            log::error!("Trying to send a packet without being connected");
        }

        Ok(())
    }

    pub async fn handle_server_auth_data(&mut self, data: &[u8]) -> std::io::Result<()> {
        let packet: ServerAuthData = match ServerAuthData::read(&mut Cursor::new(data)) {
            Ok(packet) => packet,
            Err(e) => panic!("Failed to read server auth data packet: {}", e),
        };
        log::info!("AuthData (0xFD) received");

        // Java handle differently with the server id:
        /*
        if (!"-".equals(var2))
        {
            String var5 = (new BigInteger(CryptManager.getServerIdHash(var2, var3, var4))).toString(16);
            String var6 = this.sendSessionRequest(this.mc.session.username, this.mc.session.sessionId, var5);

            if (!"ok".equalsIgnoreCase(var6))
            {
                this.netManager.networkShutdown("disconnect.loginFailedInfo", new Object[] {var6});
                return;
            }
        }
         */
        if packet.server_id != "-" {
            panic!("Server is in online mode.");
        }

        // We fully handle the packet, we can now create the packet 252 and send it
        let (packet_252, shared_secret) =
            SharedKeyPacket::new(&packet.verify_token, &packet.public_key);

        // After this packet, every packet will be encrypted
        self.send_packet(packet_252).await?;

        // shared_secret is 16 bytes. it's the key and the IV
        let secret_bytes = shared_secret.as_slice();

        //Create the décryptor and encryptor (work with AES128)
        self.encryptor = Some(
            AesCfb8Enc::new_from_slices(secret_bytes, secret_bytes)
                .expect("Invalid Key size for AES-128"),
        );

        self.decryptor = Some(
            AesCfb8Dec::new_from_slices(secret_bytes, secret_bytes)
                .expect("Invalid Key size for AES-128"),
        );

        Ok(())
    }

    pub async fn handle_shared_key(&mut self, data: &[u8]) -> std::io::Result<()> {
        // Packet to confirm if the server confirm
        let packet = SharedKeyPacket::read(&mut Cursor::new(data));
        log::info!("Shared Key Packet received");

        if packet?.is_encryption_confirmed() {
            log::info!("Encryption Confirmed");
            self.connection_state = ConnectionState::Encrypted;
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
