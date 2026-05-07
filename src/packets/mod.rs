mod packet0_keep_alive;
mod packet103_set_slot;
mod packet104_window_items;
mod packet10_flying;
mod packet132_tile_entity_data;
pub mod packet13_player_look_move;
mod packet16_block_item_switch;
mod packet1_login;
mod packet201_player_info;
mod packet202_player_abilities;
pub mod packet205_client_command;
mod packet24_mob_spawn;
pub mod packet252_shared_key;
pub mod packet253_server_auth_data;
pub mod packet254_server_ping;
pub mod packet255_kick_disconnect;
pub mod packet2_client_protocol;
mod packet4_update_time;
mod packet56_map_chunk;
mod packet6_spawn_position;
mod packet70_game_event;
pub mod packet_trait;
pub mod types;
pub mod utils;

use crate::network::connection::Encryption;
use crate::packets::InboundPacket::MobSpawn;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet1_login::LoginPacket;
use crate::packets::packet4_update_time::UpdateTimePacket;
use crate::packets::packet6_spawn_position::SpawnPositionPacket;
use crate::packets::packet13_player_look_move::PlayerLookMovePacket;
use crate::packets::packet16_block_item_switch::BlockItemSwitchPacket;
use crate::packets::packet24_mob_spawn::MobSpawnPacket;
use crate::packets::packet56_map_chunk::MapChunkPacket;
use crate::packets::packet70_game_event::GameEventPacket;
use crate::packets::packet103_set_slot::SetSlotPacket;
use crate::packets::packet104_window_items::WindowItemsPacket;
use crate::packets::packet132_tile_entity_data::TileEntityDataPacket;
use crate::packets::packet201_player_info::PlayerInfoPacket;
use crate::packets::packet202_player_abilities::PlayerAbilitiesPacket;
use crate::packets::packet252_shared_key::SharedKeyPacket;
use crate::packets::packet253_server_auth_data::ServerAuthDataPacket;
use crate::packets::utils::read_u8;
use packet0_keep_alive::KeepAlivePacket;
use std::io::{Error, ErrorKind};
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

// Sorted alphabetically
/// This enum contain all the received packet
pub enum InboundPacket {
    BlockItemSwitch(BlockItemSwitchPacket),
    GameEvent(GameEventPacket),
    KeepAlive(KeepAlivePacket),
    Login(LoginPacket),
    MapChunk(MapChunkPacket),
    MobSpawn(MobSpawnPacket),
    PlayerAbilities(PlayerAbilitiesPacket),
    PlayerInfo(PlayerInfoPacket),
    PlayerLookMove(PlayerLookMovePacket),
    ServerAuthData(ServerAuthDataPacket),
    SetSlot(SetSlotPacket),
    SharedKey(SharedKeyPacket),
    SpawnPosition(SpawnPositionPacket),
    TileEntityData(TileEntityDataPacket),
    UpdateTime(UpdateTimePacket),
    WindowItems(WindowItemsPacket),
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
                KeepAlivePacket::read(reader, encryption).await?,
            )),
            1 => Ok(InboundPacket::Login(
                LoginPacket::read(reader, encryption).await?,
            )),
            4 => Ok(InboundPacket::UpdateTime(
                UpdateTimePacket::read(reader, encryption).await?,
            )),
            6 => Ok(InboundPacket::SpawnPosition(
                SpawnPositionPacket::read(reader, encryption).await?,
            )),
            13 => Ok(InboundPacket::PlayerLookMove(
                PlayerLookMovePacket::read(reader, encryption).await?,
            )),
            16 => Ok(InboundPacket::BlockItemSwitch(
                BlockItemSwitchPacket::read(reader, encryption).await?,
            )),
            24 => Ok(MobSpawn(MobSpawnPacket::read(reader, encryption).await?)),
            56 => Ok(InboundPacket::MapChunk(
                MapChunkPacket::read(reader, encryption).await?,
            )),
            70 => Ok(InboundPacket::GameEvent(
                GameEventPacket::read(reader, encryption).await?,
            )),
            103 => Ok(InboundPacket::SetSlot(
                SetSlotPacket::read(reader, encryption).await?,
            )),
            104 => Ok(InboundPacket::WindowItems(
                WindowItemsPacket::read(reader, encryption).await?,
            )),
            132 => Ok(InboundPacket::TileEntityData(
                TileEntityDataPacket::read(reader, encryption).await?,
            )),
            201 => Ok(InboundPacket::PlayerInfo(
                PlayerInfoPacket::read(reader, encryption).await?,
            )),
            202 => Ok(InboundPacket::PlayerAbilities(
                PlayerAbilitiesPacket::read(reader, encryption).await?,
            )),
            252 => Ok(InboundPacket::SharedKey(
                SharedKeyPacket::read(reader, encryption).await?,
            )),
            253 => Ok(InboundPacket::ServerAuthData(
                ServerAuthDataPacket::read(reader, encryption).await?,
            )),

            id => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Packet ID {} unimplemented", id),
            )),
        }
    }
}
