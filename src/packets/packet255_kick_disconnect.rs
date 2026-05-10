use crate::network::connection::Encryption;
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::packets::utils::{read_string, write_string};
use bytes::{BufMut, BytesMut};
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct KickDisconnectPacket {
    pub reason: String,
}

impl KickDisconnectPacket {
    /// Get all the server infos like in the minecraft server list
    pub fn format_server_infos(&self) -> Result<ServerPingResponse, Error> {
        ServerPingResponse::from_kickdisconnect(self)
    }
}

impl ServerPacket for KickDisconnectPacket {
    /// Create the KickDisconnect packet from the entire buffer
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            reason: read_string(reader, encryption).await?,
        })
    }
}

impl ClientPacket for KickDisconnectPacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        buffer.put_u8(255);
        write_string(buffer, &self.reason)?;

        Ok(())
    }
}

pub struct ServerPingResponse {
    /// The max protocol today (Macrh 2026) is 774 (1.21.11)
    /// We don't want snapshot because they are weird (Java edition 26.1 Snapshot 9 protocol = 1073742119)
    pub protocol: u16,
    /// ex: 1.4.7
    pub server_version: String,
    pub motd: String,
    /// u16 = 65535 max, I think this should be enough
    pub player_count: u16,
    pub max_players: u16,
}

impl ServerPingResponse {
    /// Parse the KickDisconnect packet to get all the infos of the server ping response
    /// To format the server infos, we get the utf16 data from the packet 255
    /// It should look like: ```#§1\051\01.4.7\0A Minecraft Server\00\020``` (in UTF16)
    /// We can clearly see the ```\0``` are space, so it's easy.
    /// Also in 1.4.7, there is no favicon.
    /// The # is a short number that represent the length of the character chain
    /// The §1 is here to say that the version of the server is > 1.3, and you need to read thing differents thing than the previous version.
    /// Ex: the protocol version
    // TODO: read motd of version < 1.4
    pub fn from_kickdisconnect(kickdisconnect: &KickDisconnectPacket) -> Result<Self, Error> {
        // In order (everything is in UTF16 String:
        // 0: lenght of the packet + magic chain (to indicate the server is > 1.3
        // 1: protocol version (ex: 51 for 1.4.7)
        // 2: server version (ex: 1.4.7)
        // 3: MOTD (ex: A Minecraft Server)
        // 4: player count
        // 5: max player
        let split: Vec<&str> = kickdisconnect.reason.split('\0').collect();

        if split.len() < 6 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "The KickDisconnect packet is not in the right format, it should contain at least 6 parts separated by \\0".to_string(),
            ));
        }

        let protocol: u16 = split[1]
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Invalid Protocol : {}", e)))?;
        let server_version: String = split[2].to_string();
        let motd: String = split[3].to_string();
        let player_count: u16 = split[4].parse().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Invalid player count : {}", e),
            )
        })?;
        let max_players: u16 = split[5].parse().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Invalid max players : {}", e),
            )
        })?;

        Ok(Self {
            protocol,
            server_version,
            motd,
            player_count,
            max_players,
        })
    }
}

impl Display for ServerPingResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n - protocol version: {} \
            \n - server version: {} \
            \n - MOTD: {} \
            \n - Player count: {} \
            \n - max players: {} ",
            self.protocol, self.server_version, self.motd, self.player_count, self.max_players
        )
    }
}
