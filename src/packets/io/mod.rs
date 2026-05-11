use crate::network::connection::Encryption;

pub mod read;
pub mod write;

pub trait MinecraftWriteExt {
    fn write_byte_array(&mut self, byte_array: &[u8]) -> Result<(), std::io::Error>;

    fn write_string(&mut self, text: &str) -> std::io::Result<()>;

    fn write_bool(&mut self, value: bool);
}

pub trait MinecraftReadExt {
    async fn read_u8(&mut self, encryption: &mut Encryption) -> std::io::Result<u8>;

    async fn read_i8(&mut self, encryption: &mut Encryption) -> std::io::Result<i8>;

    async fn read_u16(&mut self, encryption: &mut Encryption) -> std::io::Result<u16>;

    async fn read_i16(&mut self, encryption: &mut Encryption) -> std::io::Result<i16>;

    async fn read_i32(&mut self, encryption: &mut Encryption) -> std::io::Result<i32>;

    async fn read_i64(&mut self, encryption: &mut Encryption) -> std::io::Result<i64>;

    async fn read_f32(&mut self, encryption: &mut Encryption) -> std::io::Result<f32>;

    async fn read_f64(&mut self, encryption: &mut Encryption) -> std::io::Result<f64>;

    async fn read_byte_array(&mut self, encryption: &mut Encryption) -> std::io::Result<Vec<u8>>;

    async fn read_string(&mut self, encryption: &mut Encryption) -> std::io::Result<String>;
}
