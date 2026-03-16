use crate::packets::ClientPacket;
use crate::packets::packet2_client_protocol::ClientProtocol;
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

        let (mut reader, mut writer) = stream.into_split();

        let parts: Vec<&str> = address.split(':').collect();
        let host: String = parts[0].to_string();
        let port: u32 = parts.get(1).unwrap_or(&"25565").parse::<u32>().unwrap();

        let handshake = ClientProtocol::new(51, &self.username, host, port);
        writer.write_all(handshake.write().as_slice()).await?;
        writer.flush().await?;

        // Init writer
        self.writer = Some(writer);

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
                    log::info!("received data: {:?}", received_data);
                }
                id => log::warn!("Packet {} unknown", id),
            };
        }

        Ok(())
    }

    /// Send a packet instantly
    pub async fn send_packet(&mut self, packet: impl ClientPacket) -> std::io::Result<()> {
        if let Some(writer) = &mut self.writer {
            writer.write_all(packet.write().as_slice()).await?;
            writer.flush().await?;
        } else {
            log::error!("Trying to send a packet without being connected");
        }
        Ok(())
    }
}
