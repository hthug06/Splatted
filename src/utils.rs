use std::io::{Error, ErrorKind, Result};

/// Read a String UTF16 from an u8 buffer
pub(crate) fn read_utf16_from_buffer(buffer: &[u8]) -> String {
    let mut utf16buffer: Vec<u16> = vec![];

    for chunk in buffer.chunks(2) {
        if chunk.len() == 2 {
            utf16buffer.push(u16::from_be_bytes([chunk[0], chunk[1]]));
        } else {
            utf16buffer.push(chunk[0] as u16)
        }
    }

    // We don't want to format here. Any format will be done later
    String::from_utf16_lossy(&utf16buffer)
}

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
