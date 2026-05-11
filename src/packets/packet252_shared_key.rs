use crate::network::connection::Encryption;
use crate::packets::io::{MinecraftReadExt, MinecraftWriteExt};
use crate::packets::packet_trait::{ClientPacket, ServerPacket};
use crate::protocol_version::ProtocolVersion;
use bytes::{BufMut, BytesMut};
use rand::RngCore;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use std::io::Error;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct SharedKeyPacket {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl SharedKeyPacket {
    /// When creating the packet, we're going to encrypt directly to be more clear.
    /// Create an AES Key with 16 byte (secret)
    /// Then, encrypt this secret + the token for verification
    /// Because after this, all the packet will be encrypted, we need the secret in the client class
    pub fn new(verify_token: &[u8], rsa_public_key: &RsaPublicKey) -> (Self, [u8; 16]) {
        let mut rng = rand::thread_rng();

        // Create the AES Key
        let mut shared_secret = [0u8; 16];
        rng.fill_bytes(&mut shared_secret);

        // encrypt the secret
        let encrypted_secret = rsa_public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, &shared_secret)
            .expect("Failed to encrypt shared secret");

        // encrypt the token
        let encrypted_token = rsa_public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, verify_token)
            .expect("Failed to encrypt verify token");

        let packet = Self {
            shared_secret: encrypted_secret,
            verify_token: encrypted_token,
        };

        (packet, shared_secret)
    }

    pub fn is_encryption_confirmed(&self) -> bool {
        self.verify_token.is_empty() && self.shared_secret.is_empty()
    }
}

impl ClientPacket for SharedKeyPacket {
    fn write_to(
        &self,
        buffer: &mut BytesMut,
        _protocol_version: ProtocolVersion,
    ) -> Result<(), Error> {
        // Shared Key packet ID is 252
        buffer.put_u8(252);

        // write shared_secret
        buffer.write_byte_array(&self.shared_secret)?;

        // write verify_token
        buffer.write_byte_array(&self.verify_token)?;

        Ok(())
    }
}

impl ServerPacket for SharedKeyPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
        _protocol_version: ProtocolVersion,
    ) -> Result<Self, Error> {
        let shared_secret = reader.read_byte_array(encryption).await?;
        let verify_token = reader.read_byte_array(encryption).await?;

        Ok(Self {
            shared_secret,
            verify_token,
        })
    }
}
