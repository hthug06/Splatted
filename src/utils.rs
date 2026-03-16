use std::io::{Cursor, Error, ErrorKind, Read, Result};

/// Add a string to an [u8] buffer
/// Minecraft 1.4.7 use utf16 for string so we need to convert
/// But the first thing to add to the array is the size of this u16 string
pub(crate) fn write_string(buffer: &mut Vec<u8>, text: &str) -> Result<()> {
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

/// Read a string with a cursor, so the rest of the packet can be read too.
pub(crate) fn read_string(cursor: &mut Cursor<&[u8]>) -> Result<String> {
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
pub(crate) fn read_byte_array(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>> {
    let mut len_bytes = [0u8; 2];
    cursor.read_exact(&mut len_bytes)?;
    let length = u16::from_be_bytes(len_bytes) as usize;

    let mut bytes = vec![0u8; length];
    cursor.read_exact(&mut bytes)?;
    Ok(bytes)
}
