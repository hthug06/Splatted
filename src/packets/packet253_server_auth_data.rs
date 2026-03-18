use crate::packets::ServerPacket;
use crate::packets::{read_byte_array, read_string};
use rsa::RsaPublicKey;
use rsa::pkcs8::DecodePublicKey;
use std::io::Cursor;

#[derive(Debug)]
pub struct ServerAuthData {
    pub server_id: String,
    pub public_key: RsaPublicKey,
    pub verify_token: Vec<u8>,
}

impl ServerPacket for ServerAuthData {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<ServerAuthData, std::io::Error> {
        // Everything is the same as java:
        // this.serverId = readString(par1DataInputStream, 20);
        let server_id = read_string(cursor)?;

        // this.publicKey = CryptManager.decodePublicKey(readBytesFromStream(par1DataInputStream));
        let public_key = match decode_public_key(read_byte_array(cursor)?.as_slice()) {
            Ok(public_key) => public_key,
            Err(e) => {
                panic!("Server public key decode error: {}", e)
            }
        };

        // this.verifyToken = readBytesFromStream(par1DataInputStream);
        let verify_token = read_byte_array(cursor)?;

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
    let public_key = RsaPublicKey::from_public_key_der(public_key_bytes)?;

    Ok(public_key)
}
