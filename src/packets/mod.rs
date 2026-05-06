pub mod packet0_keep_alive;
pub mod packet1_login;
mod packet202_player_abilities;
pub mod packet205_client_command;
pub mod packet252_shared_key;
pub mod packet253_server_auth_data;
pub mod packet254_server_ping;
pub mod packet255_kick_disconnect;
pub mod packet2_client_protocol;
mod packet6_spawn_position;
pub mod packet_trait;
pub mod types;
pub mod utils;

use crate::network::connection::Encryption;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet1_login::Login;
use crate::packets::packet6_spawn_position::SpawnPosition;
use crate::packets::packet202_player_abilities::PlayerAbilities;
use crate::packets::packet252_shared_key::SharedKeyPacket;
use crate::packets::packet253_server_auth_data::ServerAuthData;
use crate::packets::utils::read_u8;
use packet0_keep_alive::KeepAlive;
use std::io::{Error, ErrorKind};
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

/// this enum contain all the packet received packet
pub enum InboundPacket {
    KeepAlive(KeepAlive),
    SpawnPosition(SpawnPosition),
    PlayerAbilities(PlayerAbilities),
    SharedKey(SharedKeyPacket),
    ServerAuthData(ServerAuthData),
    Login(Login),
}

impl InboundPacket {
    /// Read the stream and return the packet That correspond
    pub async fn read_from_stream(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> std::io::Result<Self> {
        // read packet id
        let packet_id = read_u8(reader, encryption).await?;

        // Match the id to handle the right packet
        match packet_id {
            0x00 => Ok(InboundPacket::KeepAlive(
                KeepAlive::read(reader, encryption).await?,
            )),
            1 => Ok(InboundPacket::Login(Login::read(reader, encryption).await?)),
            6 => Ok(InboundPacket::SpawnPosition(
                SpawnPosition::read(reader, encryption).await?,
            )),
            202 => Ok(InboundPacket::PlayerAbilities(
                PlayerAbilities::read(reader, encryption).await?,
            )),
            252 => Ok(InboundPacket::SharedKey(
                SharedKeyPacket::read(reader, encryption).await?,
            )),
            253 => Ok(InboundPacket::ServerAuthData(
                ServerAuthData::read(reader, encryption).await?,
            )),

            id => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Packet ID {} unimplemented", id),
            )),
        }
    }
}
