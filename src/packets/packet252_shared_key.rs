use crate::packets::{ClientPacket, ServerPacket};
use rand::RngCore;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use std::io::{Cursor, Error, Read};

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
            .expect("Failed to encrypt shared secret");

        let packet = Self {
            shared_secret: encrypted_secret,
            verify_token: encrypted_token,
        };

        (packet, shared_secret)
    }

    pub fn is_encryption_confirmed(&self) -> bool {
        self.verify_token == [0, 0] && self.shared_secret == [0, 0]
    }
}

impl ClientPacket for SharedKeyPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        buffer.push(0xFC); // ID 252

        //NO
        // Écriture du secret (Taille u16 + Données)
        buffer.extend((self.shared_secret.len() as u16).to_be_bytes());
        buffer.extend(&self.shared_secret);

        // Écriture du token (Taille u16 + Données)
        buffer.extend((self.verify_token.len() as u16).to_be_bytes());
        buffer.extend(&self.verify_token);

        Ok(())
    }
}

impl ServerPacket for SharedKeyPacket {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut shared_secret: [u8; 2] = [0u8; 2];
        cursor.read_exact(&mut shared_secret)?;

        let mut verify_token: [u8; 2] = [0u8; 2];
        cursor.read_exact(&mut verify_token)?;

        Ok(Self {
            shared_secret: shared_secret.to_vec(),
            verify_token: verify_token.to_vec(),
        })
    }
}
