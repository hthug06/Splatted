use crate::packets::ClientPacket;
use crate::packets::ServerPacket;
use crate::packets::packet2_client_protocol::ClientProtocol;
use crate::packets::packet253_server_auth_data::ServerAuthData;
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedWriteHalf;

pub struct Client {
    username: String,
    /// Here, we only take a writer because the reader will always be active in the connect function.
    /// Also, we use an option because we're only going to connect the client in the connect function.
    writer: Option<OwnedWriteHalf>,
}

impl Client {
    pub fn new(username: &str) -> Client {
        Self {
            username: username.to_string(),
            writer: None,
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
        let mut buffer: [u8; 1024] = [0; 1024];

        // Listen for packet
        loop {
            let bytes_read: usize = reader.read(&mut buffer).await?;

            if bytes_read == 0 {
                continue;
            }

            let received_data: &[u8] = &buffer[..bytes_read];

            // log::info!("Received data: {:?}", received_data);

            match received_data[0] {
                253 => {
                    log::info!("AuthData (0xFD) received");
                    self.handle_server_auth_data(&received_data[1..]).await?;
                }
                id => log::warn!("Packet {} unknown", id),
            };
        }

        Ok(())
    }

    /// Send a packet to the network instantly
    pub async fn send_packet(&mut self, packet: impl ClientPacket) -> std::io::Result<()> {
        let mut buffer = Vec::new();

        packet
            .write_to(&mut buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if let Some(writer) = &mut self.writer {
            writer.write_all(&buffer).await?;
            writer.flush().await?;
        } else {
            log::error!("Trying to send a packet without being connected");
        }

        Ok(())
    }

    pub async fn handle_server_auth_data(&self, data: &[u8]) -> std::io::Result<()> {
        let packet = match ServerAuthData::read(&mut Cursor::new(data)) {
            Ok(packet) => packet,
            Err(e) => panic!("Failed to read server auth data packet: {}", e),
        };

        log::info!("Received server auth data: {:?}", packet);

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
        // TODO handle other server ids
        if packet.server_id != "-".to_string() {
            panic!("Received server auth data packet with wrong server id");
        }

        //TODO send packet252

        Ok(())
    }
}
