use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::errors::wrong_packet_error::WrongPacketError;
use crate::utils;

pub struct ServerInfo;

impl ServerInfo {
    pub async fn infos(address: &String) -> Result<(), Error> {
        // Connect to the server
        let mut stream: TcpStream = TcpStream::connect(address).await?;

        // Send first packet
        stream.write_all(&[254, 1]).await?;

        //Create a buffer
        // in this version, we can't know before the size of the buffer
        // 1024 might be good for now, but we might need to extend it when parsing chunk packet...
        // /** A temporary storage for the compressed chunk data byte array. */
        // private static byte[] temp = new byte[196864]; in source code so yeeeee...
        let mut buffer: [u8; 1024] = [0; 1024];

        // Listen for packet
        loop {
            let bytes_read = stream.read(&mut buffer).await?;


            // After receiving the serverListPacket, the connection close and we stop listening to packet
            if bytes_read == 0 {
                println!("Connection closed.");
                break;
            }
            // We only can get the server infos here
            // But if we get another packet, we throw an error
            let received_data: &[u8] = &buffer[..bytes_read];

            if received_data[0] != 254 {
                return Err(Error::new(ErrorKind::InvalidData, WrongPacketError{right_packet: 254, wrong_packet:received_data[0]}));
            }
            log::info!("Received : {:?}", &received_data[1..]);
            let utf16_data: String = utils::read_utf16_from_buffer(&received_data[1..]);
            log::info!(
                "Received + translated: {:?}",
                utf16_data
            );
        }

        Ok (())
    }
    

    fn format_server_infos(server_infos: &String) -> String {
        String::from("Server Info:\n")
    }
}
