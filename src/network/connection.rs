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
        }
    }

    /// Encrypt bytes to send in place.
    /// only if the cipher is active (Encrypted phase).
    pub fn encrypt(&mut self, buf: &mut [u8]) {
        if let Some(cipher) = &mut self.cipher {
            cipher.encryptor.encrypt(buf);
        }
    }

    /// Prepares AES-128-CFB8 encryption after receiving packet 0xFC (Shared Key).
    /// The cipher is initialized, but encryption is not active until `enable_encryption` is called.
    pub fn set_encryption(&mut self, shared_secret: &[u8; 16]) {
        self.cipher = Some(Cipher::new(shared_secret));
        self.state = EncryptionState::EncryptionPending;
        log::info!("Encryption prepared (AES-128-CFB8), waiting for activation");
    }

    /// Activate AES-128-CFB8 encryption for all future reads and writes.
    pub fn enable_encryption(&mut self) {
        self.state = EncryptionState::Encrypted;
        log::info!("Encryption enabled (AES-128-CFB8) for all I/O");
    }

    /// Check if the connection is fully encrypted
    pub fn is_encrypted(&self) -> bool {
        matches!(self.state, EncryptionState::Encrypted)
    }
}
