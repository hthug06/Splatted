/// Read a String UTF16 from an u8 buffer
/// Used with the KickPacket or to know the server infos
pub(crate) fn read_utf16_from_buffer(buffer: &[u8]) -> String {
    let mut utf16buffer: Vec<u16> = vec![];
    let mut elt1: Option<u8> = None;
    for i in 0..buffer.len() {
        // First part of u16, just store if
        if i % 2 == 0 {
            elt1 = Some(buffer[i]);
        }
        //Second part, shift and create the u16
        else {
            utf16buffer.push(u16::from_be_bytes([elt1.unwrap(), buffer[i]]));

            //reset
            elt1 = None;
        }
    }

    if let Some(final_elt) = elt1 {
        utf16buffer.push(final_elt as u16)
    }

    // Let the \0 live, so we can parse the text if this use more than 1 variable
    // Like for the server info, this return : #§1\051\01.4.7\0A Minecraft Server\01\020
    // Here we can clearly see the protocol, MOTD, player count and max player
    String::from_utf16_lossy(&utf16buffer)
}
