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
