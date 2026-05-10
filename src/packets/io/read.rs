use crate::network::connection::Encryption;
use crate::packets::io::MinecraftReadExt;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

impl MinecraftReadExt for BufReader<OwnedReadHalf> {
    /// Read an u8 (byte but unsigned)
    async fn read_u8(&mut self, encryption: &mut Encryption) -> std::io::Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf).await?;
        encryption.decrypt(&mut buf);
        Ok(buf[0])
    }

    /// Read an i8 (byte but signed)
    /// it's like reading a byte in java
    /// like par1DataInputStream.readByte() in mc code
    async fn read_i8(&mut self, encryption: &mut Encryption) -> std::io::Result<i8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf).await?;
        encryption.decrypt(&mut buf);

        Ok(i8::from_be_bytes(buf))
    }

    /// Read an u16
    /// it's like reading a short in java (but unsigned)
    /// like par1DataInputStream.readShort() in mc code
    async fn read_u16(&mut self, encryption: &mut Encryption) -> std::io::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf).await?;
        encryption.decrypt(&mut buf);
        Ok(u16::from_be_bytes(buf))
    }

    /// Read an i16
    /// it's like reading a short in java
    /// like par1DataInputStream.readShort() in mc code
    async fn read_i16(&mut self, encryption: &mut Encryption) -> std::io::Result<i16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf).await?;
        encryption.decrypt(&mut buf);
        Ok(i16::from_be_bytes(buf))
    }

    /// Read an i32
    /// it's like reading an integer in java
    /// like par1DataInputStream.readInt() in mc code
    async fn read_i32(&mut self, encryption: &mut Encryption) -> std::io::Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf).await?;
        encryption.decrypt(&mut buf);
        Ok(i32::from_be_bytes(buf))
    }

    /// Read an i64
    /// it's like reading a long in java
    /// like par1DataInputStream.readLong() in mc code
    async fn read_i64(&mut self, encryption: &mut Encryption) -> std::io::Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).await?;
        encryption.decrypt(&mut buf);
        Ok(i64::from_be_bytes(buf))
    }

    /// Read a f32
    /// it's like reading a float in java
    /// like par1DataInputStream.readFloat() in mc code
    async fn read_f32(&mut self, encryption: &mut Encryption) -> std::io::Result<f32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf).await?;
        encryption.decrypt(&mut buf);
        Ok(f32::from_be_bytes(buf))
    }

    /// Read a f64
    /// it's like reading a double in java
    /// like par1DataInputStream.readDouble() in mc code
    async fn read_f64(&mut self, encryption: &mut Encryption) -> std::io::Result<f64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).await?;
        encryption.decrypt(&mut buf);
        Ok(f64::from_be_bytes(buf))
    }

    /// Reads a Minecraft byte array (prefixed with u16 length, then the bytes)
    async fn read_byte_array(&mut self, encryption: &mut Encryption) -> std::io::Result<Vec<u8>> {
        // read length
        let mut len_bytes = [0u8; 2];
        self.read_exact(&mut len_bytes).await?;
        encryption.decrypt(&mut len_bytes);

        let length = u16::from_be_bytes(len_bytes) as usize;

        // read data
        let mut bytes = vec![0u8; length];
        if length > 0 {
            self.read_exact(&mut bytes).await?;
            encryption.decrypt(&mut bytes);
        }

        Ok(bytes)
    }

    /// Read a string (UTF16)
    async fn read_string(&mut self, encryption: &mut Encryption) -> std::io::Result<String> {
        // Read string size
        let mut len_bytes = [0u8; 2];
        self.read_exact(&mut len_bytes).await?;
        encryption.decrypt(&mut len_bytes);
        let length = u16::from_be_bytes(len_bytes) as usize;

        // From the Minecraft source code
        if length > 32_767 {
            return Err(Error::new(ErrorKind::InvalidData, "String too big"));
        }

        // Read UTF16 text (len * 2 bytes)
        let mut utf16_bytes = vec![0u8; length * 2];
        if length > 0 {
            self.read_exact(&mut utf16_bytes).await?;
            encryption.decrypt(&mut utf16_bytes);
        }

        // Convert in string
        let u16_chars: Vec<u16> = utf16_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();

        String::from_utf16(&u16_chars)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Invalid UTF-16: {:?}", e)))
    }
}
