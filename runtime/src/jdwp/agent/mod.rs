use crate::jdwp::DebugState;
use crate::jdwp::agent::command::JdwpCommand;
use crate::jdwp::agent::error_code::JdwpError;
use crate::jdwp::agent::packet::{CommandPacket, Packet, ReplyPacket};
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread::JoinHandle;

mod command;
mod error_code;
mod packet;
mod reader;

const HANDSHAKE: &[u8; 14] = b"JDWP-Handshake";

pub fn start_jdwp_agent(debug: Arc<DebugState>, port: u16) -> JoinHandle<()> {
    std::thread::spawn(move || {
        jdwp_agent_routine(debug, port);
    })
}

fn jdwp_agent_routine(debug: Arc<DebugState>, port: u16) {
    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("JDWP agent listening on port {}", port);

    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Debugger connected from {}", addr);

        if let Err(e) = perform_handshake(&mut stream) {
            eprintln!("Handshake failed: {}", e);
            continue;
        }

        debug.set_connected(true);

        handle_connection(debug.clone(), stream);

        debug.set_connected(false);
        debug.resume_all();

        println!("Debugger disconnected from {}", addr);
    }
}

fn perform_handshake(stream: &mut TcpStream) -> Result<(), JdwpError> {
    let mut buf = [0u8; 14];
    stream.read_exact(&mut buf)?;

    if &buf != HANDSHAKE {
        return Err(JdwpError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid handshake",
        )));
    }

    stream.write_all(HANDSHAKE)?;
    stream.flush()?;

    Ok(())
}

fn send_reply(stream: &mut TcpStream, reply_packet: ReplyPacket) -> Result<(), JdwpError> {
    let length = 11 + reply_packet.data.len() as u32;
    let mut buffer = Vec::with_capacity(length as usize);

    buffer.extend(&length.to_be_bytes());
    buffer.extend(&reply_packet.id.to_be_bytes());
    buffer.push(0x80); // reply flag
    buffer.extend(&reply_packet.error_code.to_be_bytes());
    buffer.extend(&reply_packet.data);

    stream.write_all(&buffer)?;
    stream.flush()?;

    Ok(())
}

fn handle_connection(debug: Arc<DebugState>, mut stream: TcpStream) {
    loop {
        match Packet::read(&mut stream) {
            Ok(Packet::Command(cmd_packet)) => match handle_command(debug.clone(), cmd_packet) {
                Ok(reply_packet) => {
                    if let Err(e) = send_reply(&mut stream, reply_packet) {
                        eprintln!("Error sending reply: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error handling command: {}", e);
                    break;
                }
            },
            Ok(Packet::Reply(_)) => unimplemented!(),
            Err(JdwpError::ConnectionClosed) => break,
            Err(e) => {
                eprintln!("Error reading packet: {}", e);
                break;
            }
        }
    }
}

fn handle_command(
    debug: Arc<DebugState>,
    cmd_packet: CommandPacket,
) -> Result<ReplyPacket, JdwpError> {
    let cmd = JdwpCommand::parse(cmd_packet.command_set, cmd_packet.command, &cmd_packet.data)?;

    println!("Received command: {:?}", cmd);

    let data = match cmd {
        JdwpCommand::VmIdSizes => Ok(handle_id_size()),
        _ => Err(JdwpError::ConnectionClosed),
    }?;

    Ok(ReplyPacket {
        id: cmd_packet.id,
        error_code: 0,
        data,
    })
}

fn handle_id_size() -> Vec<u8> {
    let mut response = Vec::new();
    response.extend(&(4i32).to_be_bytes()); // fieldID size
    response.extend(&(4i32).to_be_bytes()); // methodID size
    response.extend(&(4i32).to_be_bytes()); // objectID size
    response.extend(&(4i32).to_be_bytes()); // referenceTypeID size
    response.extend(&(4i32).to_be_bytes()); // frameID size
    response
}
