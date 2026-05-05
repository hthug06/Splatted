use crate::network::cipher::Cipher;

enum EncryptionState {
    Handshaking,
    EncryptionPending,
    Encrypted,
}

pub struct Encryption {
    state: EncryptionState,
    cipher: Option<Cipher>,
}

impl Encryption {
    pub fn new() -> Self {
        Self {
            state: EncryptionState::Handshaking,
            cipher: None,
        }
    }

    /// Decrypt received bytes in place.
    /// only if the cipher is active (Encrypted phase).
    pub fn decrypt(&mut self, buf: &mut [u8]) {
        if !self.is_encrypted() {
            return;
        }
        if let Some(cipher) = &mut self.cipher {
            cipher.decryptor.decrypt(buf);
            log::info!("Decrypted data");
        }
    }

    /// Encrypt bytes to send in place.
    /// only if the cipher is active (Encrypted phase).
    pub fn encrypt(&mut self, buf: &mut [u8]) {
        if let Some(cipher) = &mut self.cipher {
            cipher.encryptor.encrypt(buf);
        }
    }

    /// Get the shared secret to enable the encryption after the packet 252 is received
    pub fn set_encryption(&mut self, shared_secret: &[u8; 16]) {
        self.cipher = Some(Cipher::new(shared_secret));
        self.state = EncryptionState::EncryptionPending;
        log::info!("AES-128-CFB8 cipher pending");
    }

    /// Activate AES-128-CFB8 encryption for all future reads and writes.
    pub fn enable_encryption(&mut self) {
        self.state = EncryptionState::Encrypted;
        log::info!("AES-128-CFB8 cipher activated");
    }

    /// Check if the connection is fully encrypted
    pub fn is_encrypted(&self) -> bool {
        matches!(self.state, EncryptionState::Encrypted)
    }
}
