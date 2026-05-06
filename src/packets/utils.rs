use crate::network::connection::Encryption;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

/// READ FUNCTION
///
/// Read an u8 (byte but unsigned)
pub async fn read_u8(
    reader: &mut BufReader<OwnedReadHalf>,
    encryption: &mut Encryption,
) -> std::io::Result<u8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).await?;
    encryption.decrypt(&mut buf);
    Ok(buf[0])
}

/// Read an i8 (byte but signed)
/// it's like reading a byte in java
/// like par1DataInputStream.readByte() in mc code
pub async fn read_i8(
    reader: &mut BufReader<OwnedReadHalf>,
    encryption: &mut Encryption,
) -> std::io::Result<i8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).await?;
    encryption.decrypt(&mut buf);

    // Fait exactement la même chose que `buf[0] as i8`
    Ok(i8::from_be_bytes(buf))
}

/// Read an i16
/// it's like reading a short in java
/// like par1DataInputStream.readShort() in mc code
pub async fn read_i16(
    reader: &mut BufReader<OwnedReadHalf>,
    encryption: &mut Encryption,
) -> std::io::Result<i16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf).await?;
    encryption.decrypt(&mut buf);
    Ok(i16::from_be_bytes(buf))
}

/// Read an i32
/// it's like reading an integer in java
/// like par1DataInputStream.readInt() in mc code
pub async fn read_i32(
    reader: &mut BufReader<OwnedReadHalf>,
    encryption: &mut Encryption,
) -> std::io::Result<i32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf).await?;
    encryption.decrypt(&mut buf);
    Ok(i32::from_be_bytes(buf))
}

/// Read an i64
/// it's like reading a long in java
/// like par1DataInputStream.readLong() in mc code
pub async fn read_i64(
    reader: &mut BufReader<OwnedReadHalf>,
    encryption: &mut Encryption,
) -> std::io::Result<i64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf).await?;
    encryption.decrypt(&mut buf);
    Ok(i64::from_be_bytes(buf))
}

/// Reads a Minecraft byte array (prefixed with u16 length, then the bytes)
pub async fn read_byte_array(
    reader: &mut BufReader<OwnedReadHalf>,
    encryption: &mut Encryption,
) -> std::io::Result<Vec<u8>> {
    // read length
    let mut len_bytes = [0u8; 2];
    reader.read_exact(&mut len_bytes).await?;
    encryption.decrypt(&mut len_bytes);

    let length = u16::from_be_bytes(len_bytes) as usize;

    // read data
    let mut bytes = vec![0u8; length];
    if length > 0 {
        reader.read_exact(&mut bytes).await?;
        encryption.decrypt(&mut bytes);
    }

    Ok(bytes)
}

/// Read a string (UTF16)
pub async fn read_string(
    reader: &mut BufReader<OwnedReadHalf>,
    encryption: &mut Encryption,
) -> std::io::Result<String> {
    // Read string size
    let mut len_bytes = [0u8; 2];
    reader.read_exact(&mut len_bytes).await?;
    encryption.decrypt(&mut len_bytes);
    let length = u16::from_be_bytes(len_bytes) as usize;

    // From the Minecraft source code
    if length > 32_767 {
        return Err(Error::new(ErrorKind::InvalidData, "String too big"));
    }

    // Read UTF16 text (len * 2 bytes)
    let mut utf16_bytes = vec![0u8; length * 2];
    if length > 0 {
        reader.read_exact(&mut utf16_bytes).await?;
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

/// WRITE FUNCTION
///
/// Write a byte array like Minecraft 1.4.7 do (len as u16 in be_byte + data)
pub fn write_byte_array(buffer: &mut Vec<u8>, byte_array: &[u8]) {
    buffer.extend((byte_array.len() as u16).to_be_bytes());
    buffer.extend(byte_array);
}

/// Add a string to an [u8] buffer
/// Minecraft 1.4.7 use utf16 for string so we need to convert
/// But the first thing to add to the array is the size of this u16 string
pub fn write_string(buffer: &mut Vec<u8>, text: &str) -> std::io::Result<()> {
    let utf16_iter = text.encode_utf16();
    let length = utf16_iter.clone().count();

    // From the Minecraft source code
    if length > 32_767 {
        return Err(Error::new(ErrorKind::InvalidData, "String too big"));
    }

    buffer.extend_from_slice(&(length as u16).to_be_bytes());
    for c in utf16_iter {
        buffer.extend_from_slice(&c.to_be_bytes());
    }
    Ok(())
}
