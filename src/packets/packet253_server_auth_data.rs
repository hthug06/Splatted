use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use crate::packets::packet_trait::ServerPacket;
use rsa::RsaPublicKey;
use rsa::pkcs8::DecodePublicKey;
use std::io::{Error, ErrorKind};
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;

pub struct ServerAuthDataPacket {
    pub(crate) server_id: String,
    pub(crate) public_key: RsaPublicKey,
    pub(crate) verify_token: Vec<u8>,
}

impl ServerPacket for ServerAuthDataPacket {
    async fn read(
        reader: &mut BufReader<OwnedReadHalf>,
        encryption: &mut Encryption,
    ) -> Result<Self, Error> {
        // Everything is the same as java:
        // this.serverId = readString(par1DataInputStream, 20);  (server id)
        let server_id = reader.read_string(encryption).await?;

        // this.publicKey = CryptManager.decodePublicKey(readBytesFromStream(par1DataInputStream));
        let public_key_bytes = reader.read_byte_array(encryption).await?;

        let public_key = match decode_public_key(&public_key_bytes) {
            Ok(key) => key,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Server public key decode error: {}", e),
                ));
            }
        };

        // this.verifyToken = readBytesFromStream(par1DataInputStream);
        let verify_token = reader.read_byte_array(encryption).await?;

        Ok(Self {
            server_id,
            public_key,
            verify_token,
        })
    }
}

fn decode_public_key(public_key_bytes: &[u8]) -> Result<RsaPublicKey, rsa::pkcs8::spki::Error> {
    // like in java:
    // X509EncodedKeySpec var1 = new X509EncodedKeySpec(par0ArrayOfByte);
    // KeyFactory var2 = KeyFactory.getInstance("RSA");
    // return var2.generatePublic(var1);
    // BUT in rust it's better ;)
    let public_key = RsaPublicKey::from_public_key_der(public_key_bytes)?;

    Ok(public_key)
}
