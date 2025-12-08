use crate::jdwp::agent::error_code::JdwpError;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;
use std::net::TcpStream;

const FLAG_REPLY: u8 = 0x80;

#[derive(Debug, Clone)]
pub struct CommandPacket {
    pub(crate) id: u32,
    pub(crate) command_set: u8,
    pub(crate) command: u8,
    pub(crate) data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ReplyPacket {
    pub id: u32,
    pub error_code: u16,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Packet {
    Command(CommandPacket),
    Reply(ReplyPacket),
}
impl Packet {
    pub fn read(stream: &mut TcpStream) -> Result<Self, JdwpError> {
        let length = stream.read_u32::<BigEndian>()?;
        let id = stream.read_u32::<BigEndian>()?;
        let flags = stream.read_u8()?;

        if flags & FLAG_REPLY != 0 {
            let error_code = stream.read_u16::<BigEndian>()?;
            let data_len = (length as usize).saturating_sub(11);
            let mut data = vec![0u8; data_len];
            if !data.is_empty() {
                stream.read_exact(&mut data)?;
            }
            Ok(Packet::Reply(ReplyPacket {
                id,
                error_code,
                data,
            }))
        } else {
            let command_set = stream.read_u8()?;
            let command = stream.read_u8()?;
            let data_len = (length as usize).saturating_sub(11);
            let mut data = vec![0u8; data_len];
            if !data.is_empty() {
                stream.read_exact(&mut data)?;
            }
            Ok(Packet::Command(CommandPacket {
                id,
                command_set,
                command,
                data,
            }))
        }
    }
}
