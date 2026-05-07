use crate::network::connection::Encryption;
use crate::packets::InboundPacket;
use crate::packets::InboundPacket::*;
use crate::packets::packet_trait::ClientPacket;
use crate::packets::packet2_client_protocol::ClientProtocolPacket;
use crate::packets::packet205_client_command::ClientCommandPacket;
use crate::packets::packet252_shared_key::SharedKeyPacket;
use crate::packets::packet253_server_auth_data::ServerAuthDataPacket;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio::io::{AsyncWriteExt, BufReader};
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

        // Split the TCP stream into owned read and write halves.
        let (stream_reader, writer) = stream.into_split();

        // Wrap the reader in a BufReader to batch network reads.
        // This prevents expensive OS syscalls on every tiny read (like 1-byte packet IDs),
        // significantly improving parsing performance.
        let mut reader = BufReader::new(stream_reader);

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
        let handshake = ClientProtocolPacket::new(51, &self.username, host, port);
        self.send_packet(handshake).await?;

        loop {
            // Read the id and decrypt everything
            let packet =
                match InboundPacket::read_from_stream(&mut reader, &mut self.encryption).await {
                    Ok(p) => p,
                    Err(e) => {
                        log::error!("Broken stream or disconnected : {}", e);
                        break;
                    }
                };

            // handle the packet
            // Sorted alphabetically
            match packet {
                BlockChange(block_change) => {
                    log::info!("Block change packet received: {:?}", block_change);
                }
                BlockItemSwitch(block_item_switch) => {
                    log::info!("Block item switch packet received: {:?}", block_item_switch);
                    // handle block item switch (NetClientHandler.java -> handleBlockItemSwitch())
                }
                DestroyEntity(destroy_entity) => {
                    log::info!("Destroy entity packet received: {:?}", destroy_entity);
                }
                DoorChange(door_change) => {
                    log::info!("Door change packet received: {:?}", door_change);
                }
                EntityHeadRotation(entity_head_rotation) => {
                    log::info!(
                        "Entity head rotation packet received: {:?}",
                        entity_head_rotation
                    );
                }
                EntityLook(entity_look) => {
                    log::info!("Entity look packet received: {:?}", entity_look);
                }
                EntityMetadata(entity_metadata) => {
                    log::info!("Entity metadata packet received: {:?}", entity_metadata);
                }
                EntityTeleport(entity_teleport) => {
                    log::info!("Entity teleport packet received: {:?}", entity_teleport);
                }
                EntityVelocity(entity_velocity) => {
                    log::info!("Entity velocity packet received: {:?}", entity_velocity);
                }
                Experience(experience) => {
                    log::info!("Experience packet received: {:?}", experience);
                }
                GameEvent(game_event) => {
                    log::info!("Game event packet received: {:?}", game_event);
                }
                KeepAlive(keep_alive_packet) => {
                    self.send_packet(keep_alive_packet).await?;
                }
                Login(login_packet) => {
                    // Do nothing with the packet, but having information about the client is useful
                    log::info!("Login packet received: {:?}", login_packet);
                }
                MapChunk(map_chunk) => {
                    log::info!(
                        "Map chunk packet received, {} chunk(s) received",
                        map_chunk.chunk_count
                    );
                    log::info!(
                        "Chunks details: {:?}, data lenght: {}, sky light: {}",
                        map_chunk.metadata,
                        map_chunk.data_length,
                        map_chunk.sky_light_sent
                    );
                }
                MobSpawn(mob_spawn) => {
                    log::info!("Mob spawn packet received: {:?}", mob_spawn);
                }
                MultiBlockChange(multi_block_change) => {
                    log::info!(
                        "Multi block change packet received: {:?}",
                        multi_block_change
                    );
                }
                PlayerAbilities(abilities) => {
                    log::info!("Player abilities packet received: {:?}", abilities);
                }
                PlayerInfo(player_info) => {
                    log::info!("Player info packet received: {:?}", player_info);
                }
                PlayerInventory(player_inventory) => {
                    log::info!("Player inventory packet received: {:?}", player_inventory);
                }
                PlayerLookMove(player_look_move) => {
                    log::info!("Player look move packet received: {:?}", player_look_move);

                    // Resend the same packet
                    self.send_packet(player_look_move).await?;
                }
                RelEntityMove(rel_entity_move) => {
                    log::info!("Rel entity move packet received: {:?}", rel_entity_move);
                }
                RelEntityMoveLook(rel_entity_move_look) => {
                    log::info!(
                        "Rel entity move packet received: {:?}",
                        rel_entity_move_look
                    );
                }
                ServerAuthData(auth_packet) => {
                    self.handle_server_auth_data(auth_packet).await?;
                }
                SetSlot(set_slot) => {
                    log::info!("Set slot packet received: {:?}", set_slot);
                }
                SharedKey(shared_key_packet) => {
                    self.handle_shared_key(shared_key_packet).await?;
                }
                SpawnPosition(position) => {
                    log::info!("Spawn position packet received: {:?}", position);
                }
                TileEntityData(tile_entity_data) => {
                    log::info!("Tile entity data packet received: {:?}", tile_entity_data);
                }
                UpdateHealth(update_health) => {
                    log::info!("Update health packet received: {:?}", update_health);
                }
                UpdateTime(update_time) => {
                    log::info!("Update time packet received: {:?}", update_time);
                }
                WindowItems(window_items) => {
                    log::info!("Window items packet received: {:?}", window_items);
                }
            }
        }
        Ok(())
    }

    /// Send a packet to the network instantly
    pub async fn send_packet(&mut self, packet: impl ClientPacket) -> std::io::Result<()> {
        let mut buffer: Vec<u8> = Vec::new();

        packet
            .write_to(&mut buffer)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        log::info!("Sending packet ID {}", &buffer[0]);

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
    pub async fn handle_server_auth_data(
        &mut self,
        packet: ServerAuthDataPacket,
    ) -> std::io::Result<()> {
        log::info!("AuthData (253 | 0xFD) received");

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

        self.encryption.set_encryption(&secret);

        Ok(())
    }

    /// Handle the packet 252 (0xFC)
    /// First, parse the received packet.
    /// Then, if the packet is right, confirm the encryption.
    /// Finally, send the ClientCommandPacket (205) to spawn the client in the world.
    pub async fn handle_shared_key(&mut self, packet: SharedKeyPacket) -> std::io::Result<()> {
        log::info!("Shared Key Packet (252 | 0xFC) received");

        // From now, every sent and received packet will be encrypted
        if packet.is_encryption_confirmed() {
            self.encryption.enable_encryption()
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid shared key, bad encryption",
            ));
        }
        self.send_packet(ClientCommandPacket::new(0)).await?;

        Ok(())
    }
}
