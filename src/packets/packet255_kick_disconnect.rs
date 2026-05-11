use crate::network::connection::Encryption;
use crate::packets::io::{MinecraftReadExt, MinecraftWriteExt};
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
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
            reason: reader.read_string(encryption).await?,
        })
    }
}

impl ClientPacket for KickDisconnectPacket {
    fn write_to(&self, buffer: &mut BytesMut) -> Result<(), Error> {
        buffer.put_u8(255);
        buffer.write_string(&self.reason)?;

        Ok(())
    }
}
#[derive(Debug)]
pub struct ServerPingResponse {
    /// The max protocol today (Macrh 2026) is 774 (1.21.11)
    /// We don't want snapshot because they are weird (Java edition 26.1 Snapshot 9 protocol = 1073742119)
    pub protocol: Option<u16>,
    /// ex: 1.4.7
    pub server_version: Option<String>,
    pub motd: String,
    /// u16 = 65535 max, I think this should be enough
    pub player_count: u16,
    pub max_players: u16,
}

impl ServerPingResponse {
    /// Parse the KickDisconnect packet to get all the infos of the server ping response
    /// To format the server infos, we get the utf16 data from the packet 255
    /// It should look like:
    /// - ```#§1\051\01.4.7\0A Minecraft Server\00\020``` (in UTF16 | 1.4-1.6). We can clearly see the ```\0``` are space, so it's easy.
    /// - ```A\0Minecraft\0Server§0§20``` (in UTF16 | 1.2-1.3). We can clearly see the ```\0``` and ```§0``` are space, so it's easy.
    ///
    /// Before the 1.7, there is no favicon.
    ///
    /// The §1 is here to say that the version of the server is > 1.3, and you need to read thing differents thing than the previous version.
    pub fn from_kickdisconnect(kickdisconnect: &KickDisconnectPacket) -> Result<Self, Error> {
        let reason = &kickdisconnect.reason;

        // Temp variable to parse the brut string of the packet
        let protocol_str: Option<&str>;
        let server_version_str: Option<&str>;
        let motd_str: &str;
        let player_count_str: &str;
        let max_players_str: &str;

        // Identity the version range
        //
        // For 1.4 -> 1.6.4:
        // In order (everything is in UTF16 String:
        // 0: length of the packet + magic chain (to indicate the server is > 1.3)
        // 1: protocol version (ex: 51 for 1.4.7)
        // 2: server version (ex: 1.4.7)
        // 3: MOTD (ex: A Minecraft Server)
        // 4: player count
        // 5: max player
        if reason.starts_with("§1\0") {
            let parts: Vec<&str> = reason.split('\0').collect();
            if parts.len() < 6 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "The KickDisconnect packet (1.4-1.6) should contain at least 6 parts separated by \\0",
                ));
            }
            protocol_str = Some(parts[1]);
            server_version_str = Some(parts[2]);
            motd_str = parts[3];
            player_count_str = parts[4];
            max_players_str = parts[5];
        }
        // For 1.2 -> 1.3.2:
        // 0: The MOTD (space = \0, split before)
        // 1: Player count
        // 2: Max player
        // separated with § for some reason
        else {
            let parts: Vec<&str> = reason.split('§').collect();
            if parts.len() < 3 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "The KickDisconnect packet (1.2-1.3) should contain at least 3 parts separated by §",
                ));
            }
            protocol_str = None;
            server_version_str = None;
            motd_str = parts[0];
            player_count_str = parts[1];
            max_players_str = parts[2];
        }

        fn parse_u16(s: &str, field_name: &str) -> Result<u16, Error> {
            s.parse().map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid {} : {}", field_name, e),
                )
            })
        }

        // Parse the protocol here because he can be None
        let protocol = match protocol_str {
            Some(p) => Some(parse_u16(p, "Protocol")?),
            None => None,
        };

        let server_infos = Self {
            protocol,
            server_version: server_version_str.map(|s| s.to_string()),
            motd: motd_str.to_string(),
            player_count: parse_u16(player_count_str, "player count")?,
            max_players: parse_u16(max_players_str, "max players")?,
        };

        Ok(server_infos)
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
            self.protocol
                .unwrap_or(0)
                .to_string()
                .replace("0", "Unknow"),
            self.server_version.clone().unwrap_or(String::from("< 1.4")),
            self.motd,
            self.player_count,
            self.max_players
        )
    }
}
