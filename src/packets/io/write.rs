use crate::packets::io::MinecraftWriteExt;
use bytes::{BufMut, BytesMut};

impl MinecraftWriteExt for BytesMut {
    /// Write a byte array like Minecraft 1.4.7 do (len as u16 in be_byte + data)
    fn write_byte_array(&mut self, byte_array: &[u8]) -> Result<(), std::io::Error> {
        // Minecraft server use short, so 32_767.
        // We need to ensure the len of the byte_array is <= 32767 else the server will crash
        if byte_array.len() > 32_767 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Byte array too big",
            ));
        }
        self.extend((byte_array.len() as u16).to_be_bytes());
        self.extend(byte_array);

        Ok(())
    }

    /// Add a string to an [u8] &mut self
    /// Minecraft 1.4.7 use utf16 for string so we need to convert
    /// But the first thing to add to the array is the size of this u16 string
    fn write_string(&mut self, text: &str) -> std::io::Result<()> {
        let utf16_data: Vec<u16> = text.encode_utf16().collect();

        // Keep the usize
        let length = utf16_data.len();

        //Verify now
        // Minecraft server use short, so 32_767.
        // We need to ensure the len of the byte_array is <= 32767 else the server will crash
        if length > 32_767 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "String too big",
            ));
        }

        // And now cast as u16
        self.extend(&(length as u16).to_be_bytes());

        for char in utf16_data {
            self.extend(char.to_be_bytes());
        }

        Ok(())
    }

    fn write_bool(&mut self, value: bool) {
        self.put_u8(if value { 1 } else { 0 });
    }
}
