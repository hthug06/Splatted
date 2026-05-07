mod packet0_keep_alive;
mod packet103_set_slot;
mod packet104_window_items;
mod packet10_flying;
mod packet132_tile_entity_data;
pub mod packet13_player_look_move;
mod packet16_block_item_switch;
mod packet18_animation;
mod packet1_login;
mod packet200_statistic;
mod packet201_player_info;
mod packet202_player_abilities;
pub mod packet205_client_command;
mod packet20_named_entity_spawn;
mod packet22_collect;
mod packet23_vehicule_spawn;
mod packet24_mob_spawn;
pub mod packet252_shared_key;
pub mod packet253_server_auth_data;
pub mod packet254_server_ping;
pub mod packet255_kick_disconnect;
mod packet25_entity_painting;
mod packet28_entity_velocity;
mod packet29_destroy_entity;
pub mod packet2_client_protocol;
mod packet30_entity;
mod packet31_rel_entity_move;
mod packet32_entity_look;
mod packet33_rel_entity_move_look;
mod packet34_entity_teleport;
mod packet35_entity_head_rotation;
mod packet38_entity_status;
mod packet3_chat;
mod packet40_entity_metadata;
mod packet43_experience;
mod packet4_update_time;
mod packet52_multi_block_change;
mod packet53_block_change;
mod packet56_map_chunk;
mod packet5_player_inventory;
mod packet61_door_change;
mod packet62_level_sound;
mod packet6_spawn_position;
mod packet70_game_event;
mod packet8_update_health;
pub mod packet_trait;
pub mod types;
pub mod utils;

use crate::network::connection::Encryption;
use crate::packets::InboundPacket::*;
use crate::packets::packet_trait::ServerPacket;
use crate::packets::packet1_login::LoginPacket;
use crate::packets::packet3_chat::ChatPacket;
use crate::packets::packet4_update_time::UpdateTimePacket;
use crate::packets::packet5_player_inventory::PlayerInventoryPacket;
use crate::packets::packet6_spawn_position::SpawnPositionPacket;
use crate::packets::packet8_update_health::UpdateHealthPacket;
use crate::packets::packet13_player_look_move::PlayerLookMovePacket;
use crate::packets::packet16_block_item_switch::BlockItemSwitchPacket;
use crate::packets::packet18_animation::AnimationPacket;
use crate::packets::packet20_named_entity_spawn::NamedEntitySpawnPacket;
use crate::packets::packet22_collect::CollectPacket;
use crate::packets::packet23_vehicule_spawn::VehicleSpawnPacket;
use crate::packets::packet24_mob_spawn::MobSpawnPacket;
use crate::packets::packet25_entity_painting::EntityPaintingPacket;
use crate::packets::packet28_entity_velocity::EntityVelocityPacket;
use crate::packets::packet29_destroy_entity::DestroyEntityPacket;
use crate::packets::packet31_rel_entity_move::RelEntityMovePacket;
use crate::packets::packet32_entity_look::EntityLookPacket;
use crate::packets::packet33_rel_entity_move_look::RelEntityMoveLookPacket;
use crate::packets::packet34_entity_teleport::EntityTeleportPacket;
use crate::packets::packet35_entity_head_rotation::EntityHeadRotationPacket;
use crate::packets::packet38_entity_status::EntityStatusPacket;
use crate::packets::packet40_entity_metadata::EntityMetadataPacket;
use crate::packets::packet43_experience::ExperiencePacket;
use crate::packets::packet52_multi_block_change::MultiBlockChangePacket;
use crate::packets::packet53_block_change::BlockChangePacket;
use crate::packets::packet56_map_chunk::MapChunkPacket;
use crate::packets::packet61_door_change::DoorChangePacket;
use crate::packets::packet62_level_sound::LevelSoundPacket;
use crate::packets::packet70_game_event::GameEventPacket;
use crate::packets::packet103_set_slot::SetSlotPacket;
use crate::packets::packet104_window_items::WindowItemsPacket;
use crate::packets::packet132_tile_entity_data::TileEntityDataPacket;
use crate::packets::packet200_statistic::StatisticPacket;
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
    Animation(AnimationPacket),
    BlockChange(BlockChangePacket),
    BlockItemSwitch(BlockItemSwitchPacket),
    Chat(ChatPacket),
    Collected(CollectPacket),
    DestroyEntity(DestroyEntityPacket),
    DoorChange(DoorChangePacket),
    EntityHeadRotation(EntityHeadRotationPacket),
    EntityLook(EntityLookPacket),
    EntityMetadata(EntityMetadataPacket),
    EntityPainting(EntityPaintingPacket),
    EntityStatus(EntityStatusPacket),
    EntityTeleport(EntityTeleportPacket),
    EntityVelocity(EntityVelocityPacket),
    Experience(ExperiencePacket),
    GameEvent(GameEventPacket),
    KeepAlive(KeepAlivePacket),
    LevelSound(LevelSoundPacket),
    Login(LoginPacket),
    MapChunk(MapChunkPacket),
    MobSpawn(MobSpawnPacket),
    MultiBlockChange(MultiBlockChangePacket),
    NamedEntitySpawn(NamedEntitySpawnPacket),
    PlayerAbilities(PlayerAbilitiesPacket),
    PlayerInfo(PlayerInfoPacket),
    PlayerInventory(PlayerInventoryPacket),
    PlayerLookMove(PlayerLookMovePacket),
    RelEntityMove(RelEntityMovePacket),
    RelEntityMoveLook(RelEntityMoveLookPacket),
    ServerAuthData(ServerAuthDataPacket),
    SetSlot(SetSlotPacket),
    SharedKey(SharedKeyPacket),
    SpawnPosition(SpawnPositionPacket),
    Statistic(StatisticPacket),
    TileEntityData(TileEntityDataPacket),
    UpdateHealth(UpdateHealthPacket),
    UpdateTime(UpdateTimePacket),
    VehiculeSpawn(VehicleSpawnPacket),
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
            0x00 => Ok(KeepAlive(KeepAlivePacket::read(reader, encryption).await?)),
            1 => Ok(Login(LoginPacket::read(reader, encryption).await?)),
            3 => Ok(Chat(ChatPacket::read(reader, encryption).await?)),
            4 => Ok(UpdateTime(
                UpdateTimePacket::read(reader, encryption).await?,
            )),
            5 => Ok(PlayerInventory(
                PlayerInventoryPacket::read(reader, encryption).await?,
            )),
            6 => Ok(SpawnPosition(
                SpawnPositionPacket::read(reader, encryption).await?,
            )),
            8 => Ok(UpdateHealth(
                UpdateHealthPacket::read(reader, encryption).await?,
            )),
            13 => Ok(PlayerLookMove(
                PlayerLookMovePacket::read(reader, encryption).await?,
            )),
            16 => Ok(BlockItemSwitch(
                BlockItemSwitchPacket::read(reader, encryption).await?,
            )),
            18 => Ok(Animation(AnimationPacket::read(reader, encryption).await?)),
            20 => Ok(NamedEntitySpawn(
                NamedEntitySpawnPacket::read(reader, encryption).await?,
            )),
            22 => Ok(Collected(CollectPacket::read(reader, encryption).await?)),
            23 => Ok(VehiculeSpawn(
                VehicleSpawnPacket::read(reader, encryption).await?,
            )),
            24 => Ok(MobSpawn(MobSpawnPacket::read(reader, encryption).await?)),
            25 => Ok(EntityPainting(
                EntityPaintingPacket::read(reader, encryption).await?,
            )),
            28 => Ok(EntityVelocity(
                EntityVelocityPacket::read(reader, encryption).await?,
            )),
            29 => Ok(DestroyEntity(
                DestroyEntityPacket::read(reader, encryption).await?,
            )),
            31 => Ok(RelEntityMove(
                RelEntityMovePacket::read(reader, encryption).await?,
            )),
            32 => Ok(EntityLook(
                EntityLookPacket::read(reader, encryption).await?,
            )),
            33 => Ok(RelEntityMoveLook(
                RelEntityMoveLookPacket::read(reader, encryption).await?,
            )),
            34 => Ok(EntityTeleport(
                EntityTeleportPacket::read(reader, encryption).await?,
            )),
            35 => Ok(EntityHeadRotation(
                EntityHeadRotationPacket::read(reader, encryption).await?,
            )),
            38 => Ok(EntityStatus(
                EntityStatusPacket::read(reader, encryption).await?,
            )),
            40 => Ok(EntityMetadata(
                EntityMetadataPacket::read(reader, encryption).await?,
            )),
            43 => Ok(Experience(
                ExperiencePacket::read(reader, encryption).await?,
            )),
            52 => Ok(MultiBlockChange(
                MultiBlockChangePacket::read(reader, encryption).await?,
            )),
            53 => Ok(BlockChange(
                BlockChangePacket::read(reader, encryption).await?,
            )),
            56 => Ok(MapChunk(MapChunkPacket::read(reader, encryption).await?)),
            61 => Ok(DoorChange(
                DoorChangePacket::read(reader, encryption).await?,
            )),
            62 => Ok(LevelSound(
                LevelSoundPacket::read(reader, encryption).await?,
            )),
            70 => Ok(GameEvent(GameEventPacket::read(reader, encryption).await?)),
            103 => Ok(SetSlot(SetSlotPacket::read(reader, encryption).await?)),
            104 => Ok(WindowItems(
                WindowItemsPacket::read(reader, encryption).await?,
            )),
            132 => Ok(TileEntityData(
                TileEntityDataPacket::read(reader, encryption).await?,
            )),
            200 => Ok(Statistic(StatisticPacket::read(reader, encryption).await?)),
            201 => Ok(PlayerInfo(
                PlayerInfoPacket::read(reader, encryption).await?,
            )),
            202 => Ok(PlayerAbilities(
                PlayerAbilitiesPacket::read(reader, encryption).await?,
            )),
            252 => Ok(SharedKey(SharedKeyPacket::read(reader, encryption).await?)),
            253 => Ok(ServerAuthData(
                ServerAuthDataPacket::read(reader, encryption).await?,
            )),

            id => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Packet ID {} unimplemented", id),
            )),
        }
    }
}
