use aes::Aes128;
use aes::cipher::KeyIvInit;
use cfb8::{Decryptor, Encryptor};

type Aes128Cfb8Enc = Encryptor<Aes128>;
type Aes128Cfb8Dec = Decryptor<Aes128>;

pub struct Cipher {
    pub(crate) encryptor: Aes128Cfb8Enc,
    pub(crate) decryptor: Aes128Cfb8Dec,
}

impl Cipher {
    pub fn new(shared_secret: &[u8; 16]) -> Self {
        Self {
            encryptor: Aes128Cfb8Enc::new(shared_secret.into(), shared_secret.into()),
            decryptor: Aes128Cfb8Dec::new(shared_secret.into(), shared_secret.into()),
        }
    }
}
