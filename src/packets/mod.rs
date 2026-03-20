use std::io::{Cursor, Error, ErrorKind, Read};

pub mod packet205_client_command;
pub mod packet252_shared_key;
pub mod packet253_server_auth_data;
pub mod packet254_server_ping;
pub mod packet255_kick_disconnect;
pub mod packet2_client_protocol;

pub trait ServerPacket {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait ClientPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) -> Result<(), Error>;
}

/// Add a string to an [u8] buffer
/// Minecraft 1.4.7 use utf16 for string so we need to convert
/// But the first thing to add to the array is the size of this u16 string
pub(crate) fn write_string(buffer: &mut Vec<u8>, text: &str) -> std::io::Result<()> {
    let utf16_iter = text.encode_utf16();
    let length = utf16_iter.clone().count();

    // From the minecraft source code
    if length > 32_767 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "String too big (exceeds 32767 characters)",
        ));
    }

    buffer.extend_from_slice(&(length as u16).to_be_bytes());

    for c in utf16_iter {
        buffer.extend_from_slice(&c.to_be_bytes());
    }

    Ok(())
}

/// Write a byte array like Minecraft 1.4.7 do (len as u16 in be_byte + data)
pub(crate) fn write_byte_array(buffer: &mut Vec<u8>, byte_array: &Vec<u8>) {
    buffer.extend((byte_array.len() as u16).to_be_bytes());
    buffer.extend(byte_array);
}

/// Read a string with a cursor, so the rest of the packet can be read too.
pub(crate) fn read_string(cursor: &mut Cursor<&[u8]>) -> std::io::Result<String> {
    // Read the size
    let mut len_bytes = [0u8; 2];
    cursor.read_exact(&mut len_bytes)?;
    let length = u16::from_be_bytes(len_bytes) as usize;

    // read all the byte (as utf16)
    let mut utf16_bytes = vec![0u8; length * 2];
    cursor.read_exact(&mut utf16_bytes)?;

    // Convert u8 into u16
    let u16_chars: Vec<u16> = utf16_bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect();

    String::from_utf16(&u16_chars).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("error on read_string: {:?}", e),
        )
    })
}

/// Reads a Minecraft byte array (prefixed with u16 length, then the bytes)
pub(crate) fn read_byte_array(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Vec<u8>> {
    let mut len_bytes = [0u8; 2];
    cursor.read_exact(&mut len_bytes)?;
    let length = u16::from_be_bytes(len_bytes) as usize;

    let mut bytes = vec![0u8; length];
    cursor.read_exact(&mut bytes)?;
    Ok(bytes)
}
