pub mod io;
mod packet0_keep_alive;
mod packet103_set_slot;
mod packet104_window_items;
mod packet10_flying;
mod packet11_player_position;
mod packet12_player_look;
mod packet130_update_sign;
mod packet132_tile_entity_data;
pub mod packet13_player_look_move;
mod packet16_block_item_switch;
mod packet18_animation;
mod packet1_login;
mod packet200_statistic;
mod packet201_player_info;
mod packet202_player_abilities;
mod packet204_client_info;
pub mod packet205_client_command;
mod packet20_named_entity_spawn;
mod packet22_collect;
mod packet23_vehicle_spawn;
mod packet24_mob_spawn;
mod packet250_custom_payload;
pub mod packet252_shared_key;
pub mod packet253_server_auth_data;
pub mod packet254_server_ping;
pub mod packet255_kick_disconnect;
mod packet25_entity_painting;
mod packet26_entity_exp_orb;
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
mod packet39_attach_entity;
mod packet3_chat;
mod packet40_entity_metadata;
mod packet41_entity_effect;
mod packet42_remove_entity_effect;
mod packet43_experience;
mod packet4_update_time;
mod packet50_pre_chunk;
mod packet51_map_chunk;
mod packet52_multi_block_change;
mod packet53_block_change;
mod packet54_play_note_block;
mod packet55_block_destroy;
mod packet56_map_chunks;
mod packet5_player_inventory;
mod packet60_explosion;
mod packet61_door_change;
mod packet62_level_sound;
mod packet6_spawn_position;
mod packet70_game_event;
mod packet71_weather;
mod packet8_update_health;
pub mod packet_trait;
pub mod types;

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
use crate::packets::packet23_vehicle_spawn::VehicleSpawnPacket;
use crate::packets::packet24_mob_spawn::MobSpawnPacket;
use crate::packets::packet25_entity_painting::EntityPaintingPacket;
use crate::packets::packet26_entity_exp_orb::EntityExpOrbPacket;
use crate::packets::packet28_entity_velocity::EntityVelocityPacket;
use crate::packets::packet29_destroy_entity::DestroyEntityPacket;
use crate::packets::packet31_rel_entity_move::RelEntityMovePacket;
use crate::packets::packet32_entity_look::EntityLookPacket;
use crate::packets::packet33_rel_entity_move_look::RelEntityMoveLookPacket;
use crate::packets::packet34_entity_teleport::EntityTeleportPacket;
use crate::packets::packet35_entity_head_rotation::EntityHeadRotationPacket;
use crate::packets::packet38_entity_status::EntityStatusPacket;
use crate::packets::packet39_attach_entity::AttachEntityPacket;
use crate::packets::packet40_entity_metadata::EntityMetadataPacket;
use crate::packets::packet41_entity_effect::EntityEffectPacket;
use crate::packets::packet42_remove_entity_effect::RemoveEntityEffectPacket;
use crate::packets::packet43_experience::ExperiencePacket;
use crate::packets::packet50_pre_chunk::PreChunkPacket;
use crate::packets::packet51_map_chunk::MapChunkPacket;
use crate::packets::packet52_multi_block_change::MultiBlockChangePacket;
use crate::packets::packet53_block_change::BlockChangePacket;
use crate::packets::packet54_play_note_block::PlayNoteBlockPacket;
use crate::packets::packet55_block_destroy::BlockDestroyPacket;
use crate::packets::packet56_map_chunks::MapChunksPacket;
use crate::packets::packet60_explosion::ExplosionPacket;
use crate::packets::packet61_door_change::DoorChangePacket;
use crate::packets::packet62_level_sound::LevelSoundPacket;
use crate::packets::packet70_game_event::GameEventPacket;
use crate::packets::packet71_weather::WeatherPacket;
use crate::packets::packet103_set_slot::SetSlotPacket;
use crate::packets::packet104_window_items::WindowItemsPacket;
use crate::packets::packet130_update_sign::UpdateSignPacket;
use crate::packets::packet132_tile_entity_data::TileEntityDataPacket;
use crate::packets::packet200_statistic::StatisticPacket;
use crate::packets::packet201_player_info::PlayerInfoPacket;
use crate::packets::packet202_player_abilities::PlayerAbilitiesPacket;
use crate::packets::packet250_custom_payload::CustomPayloadPacket;
use crate::packets::packet252_shared_key::SharedKeyPacket;
use crate::packets::packet253_server_auth_data::ServerAuthDataPacket;
use crate::packets::packet255_kick_disconnect::KickDisconnectPacket;
use crate::protocol_version::ProtocolVersion;
use io::MinecraftReadExt;
use log::info;
use packet0_keep_alive::KeepAlivePacket;
use std::io::{Error, ErrorKind};
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

// Sorted alphabetically
/// This enum contain all the received packet
pub enum InboundPacket {
    Animation(AnimationPacket),
    AttachEntity(AttachEntityPacket),
    BlockChange(BlockChangePacket),
    BlockDestroy(BlockDestroyPacket),
    BlockItemSwitch(BlockItemSwitchPacket),
    Chat(ChatPacket),
    Collected(CollectPacket),
    CustomPayload(CustomPayloadPacket),
    DestroyEntity(DestroyEntityPacket),
    DoorChange(DoorChangePacket),
    EntityEffect(EntityEffectPacket),
    EntityExpOrb(EntityExpOrbPacket),
    EntityHeadRotation(EntityHeadRotationPacket),
    EntityLook(EntityLookPacket),
    EntityMetadata(EntityMetadataPacket),
    EntityPainting(EntityPaintingPacket),
    EntityStatus(EntityStatusPacket),
    EntityTeleport(EntityTeleportPacket),
    EntityVelocity(EntityVelocityPacket),
    Experience(ExperiencePacket),
    Explosion(ExplosionPacket),
    GameEvent(GameEventPacket),
    KeepAlive(KeepAlivePacket),
    KickDisconnect(KickDisconnectPacket),
    LevelSound(LevelSoundPacket),
    Login(LoginPacket),
    MapChunk(MapChunkPacket),
    MapChunks(MapChunksPacket),
    MobSpawn(MobSpawnPacket),
    MultiBlockChange(MultiBlockChangePacket),
    NamedEntitySpawn(NamedEntitySpawnPacket),
    PlayerAbilities(PlayerAbilitiesPacket),
    PlayerInfo(PlayerInfoPacket),
    PlayerInventory(PlayerInventoryPacket),
    PlayerLookMove(PlayerLookMovePacket),
    PlayNoteBlock(PlayNoteBlockPacket),
    PreChunk(PreChunkPacket),
    RelEntityMove(RelEntityMovePacket),
    RelEntityMoveLook(RelEntityMoveLookPacket),
    RemoveEntityEffect(RemoveEntityEffectPacket),
    ServerAuthData(ServerAuthDataPacket),
    SetSlot(SetSlotPacket),
    SharedKey(SharedKeyPacket),
    SpawnPosition(SpawnPositionPacket),
    Statistic(StatisticPacket),
    TileEntityData(TileEntityDataPacket),
    UpdateHealth(UpdateHealthPacket),
    UpdateSign(UpdateSignPacket),
    UpdateTime(UpdateTimePacket),
    VehicleSpawn(VehicleSpawnPacket),
    Weather(WeatherPacket),
    WindowItems(WindowItemsPacket),
}

impl InboundPacket {
    /// Read the stream and return the packet That correspond
    pub async fn read_from_stream(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        protocol_version: ProtocolVersion,
    ) -> std::io::Result<Self> {
        // read packet id
        let packet_id = reader.read_u8(encryption).await?;
        info!("Received packet with id: {}", packet_id);

        // Match the id to handle the right packet
        match packet_id {
            0x00 => Ok(KeepAlive(
                KeepAlivePacket::read(reader, encryption, protocol_version).await?,
            )),
            1 => Ok(Login(
                LoginPacket::read(reader, encryption, protocol_version).await?,
            )),
            3 => Ok(Chat(
                ChatPacket::read(reader, encryption, protocol_version).await?,
            )),
            4 => Ok(UpdateTime(
                UpdateTimePacket::read(reader, encryption, protocol_version).await?,
            )),
            5 => Ok(PlayerInventory(
                PlayerInventoryPacket::read(reader, encryption, protocol_version).await?,
            )),
            6 => Ok(SpawnPosition(
                SpawnPositionPacket::read(reader, encryption, protocol_version).await?,
            )),
            8 => Ok(UpdateHealth(
                UpdateHealthPacket::read(reader, encryption, protocol_version).await?,
            )),
            13 => Ok(PlayerLookMove(
                PlayerLookMovePacket::read(reader, encryption, protocol_version).await?,
            )),
            16 => Ok(BlockItemSwitch(
                BlockItemSwitchPacket::read(reader, encryption, protocol_version).await?,
            )),
            18 => Ok(Animation(
                AnimationPacket::read(reader, encryption, protocol_version).await?,
            )),
            20 => Ok(NamedEntitySpawn(
                NamedEntitySpawnPacket::read(reader, encryption, protocol_version).await?,
            )),
            22 => Ok(Collected(
                CollectPacket::read(reader, encryption, protocol_version).await?,
            )),
            23 => Ok(VehicleSpawn(
                VehicleSpawnPacket::read(reader, encryption, protocol_version).await?,
            )),
            24 => Ok(MobSpawn(
                MobSpawnPacket::read(reader, encryption, protocol_version).await?,
            )),
            25 => Ok(EntityPainting(
                EntityPaintingPacket::read(reader, encryption, protocol_version).await?,
            )),
            26 => Ok(EntityExpOrb(
                EntityExpOrbPacket::read(reader, encryption, protocol_version).await?,
            )),
            28 => Ok(EntityVelocity(
                EntityVelocityPacket::read(reader, encryption, protocol_version).await?,
            )),
            29 => Ok(DestroyEntity(
                DestroyEntityPacket::read(reader, encryption, protocol_version).await?,
            )),
            31 => Ok(RelEntityMove(
                RelEntityMovePacket::read(reader, encryption, protocol_version).await?,
            )),
            32 => Ok(EntityLook(
                EntityLookPacket::read(reader, encryption, protocol_version).await?,
            )),
            33 => Ok(RelEntityMoveLook(
                RelEntityMoveLookPacket::read(reader, encryption, protocol_version).await?,
            )),
            34 => Ok(EntityTeleport(
                EntityTeleportPacket::read(reader, encryption, protocol_version).await?,
            )),
            35 => Ok(EntityHeadRotation(
                EntityHeadRotationPacket::read(reader, encryption, protocol_version).await?,
            )),
            38 => Ok(EntityStatus(
                EntityStatusPacket::read(reader, encryption, protocol_version).await?,
            )),
            39 => Ok(AttachEntity(
                AttachEntityPacket::read(reader, encryption, protocol_version).await?,
            )),
            40 => Ok(EntityMetadata(
                EntityMetadataPacket::read(reader, encryption, protocol_version).await?,
            )),
            41 => Ok(EntityEffect(
                EntityEffectPacket::read(reader, encryption, protocol_version).await?,
            )),
            42 => Ok(RemoveEntityEffect(
                RemoveEntityEffectPacket::read(reader, encryption, protocol_version).await?,
            )),
            43 => Ok(Experience(
                ExperiencePacket::read(reader, encryption, protocol_version).await?,
            )),
            50 => Ok(PreChunk(
                PreChunkPacket::read(reader, encryption, protocol_version).await?,
            )),
            51 => Ok(MapChunk(
                MapChunkPacket::read(reader, encryption, protocol_version).await?,
            )),
            52 => Ok(MultiBlockChange(
                MultiBlockChangePacket::read(reader, encryption, protocol_version).await?,
            )),
            53 => Ok(BlockChange(
                BlockChangePacket::read(reader, encryption, protocol_version).await?,
            )),
            54 => Ok(PlayNoteBlock(
                PlayNoteBlockPacket::read(reader, encryption, protocol_version).await?,
            )),
            55 => Ok(BlockDestroy(
                BlockDestroyPacket::read(reader, encryption, protocol_version).await?,
            )),
            56 => Ok(MapChunks(
                MapChunksPacket::read(reader, encryption, protocol_version).await?,
            )),
            60 => Ok(Explosion(
                ExplosionPacket::read(reader, encryption, protocol_version).await?,
            )),
            61 => Ok(DoorChange(
                DoorChangePacket::read(reader, encryption, protocol_version).await?,
            )),
            62 => Ok(LevelSound(
                LevelSoundPacket::read(reader, encryption, protocol_version).await?,
            )),
            70 => Ok(GameEvent(
                GameEventPacket::read(reader, encryption, protocol_version).await?,
            )),
            71 => Ok(Weather(
                WeatherPacket::read(reader, encryption, protocol_version).await?,
            )),
            103 => Ok(SetSlot(
                SetSlotPacket::read(reader, encryption, protocol_version).await?,
            )),
            104 => Ok(WindowItems(
                WindowItemsPacket::read(reader, encryption, protocol_version).await?,
            )),
            130 => Ok(UpdateSign(
                UpdateSignPacket::read(reader, encryption, protocol_version).await?,
            )),
            132 => Ok(TileEntityData(
                TileEntityDataPacket::read(reader, encryption, protocol_version).await?,
            )),
            200 => Ok(Statistic(
                StatisticPacket::read(reader, encryption, protocol_version).await?,
            )),
            201 => Ok(PlayerInfo(
                PlayerInfoPacket::read(reader, encryption, protocol_version).await?,
            )),
            202 => Ok(PlayerAbilities(
                PlayerAbilitiesPacket::read(reader, encryption, protocol_version).await?,
            )),
            250 => Ok(CustomPayload(
                CustomPayloadPacket::read(reader, encryption, protocol_version).await?,
            )),
            252 => Ok(SharedKey(
                SharedKeyPacket::read(reader, encryption, protocol_version).await?,
            )),
            253 => Ok(ServerAuthData(
                ServerAuthDataPacket::read(reader, encryption, protocol_version).await?,
            )),
            255 => Ok(KickDisconnect(
                KickDisconnectPacket::read(reader, encryption, protocol_version).await?,
            )),

            id => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Packet ID {} unimplemented", id),
            )),
        }
    }
}
