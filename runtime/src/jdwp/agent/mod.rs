use crate::jdwp::DebugState;
use crate::jdwp::agent::command::JdwpCommand;
use crate::jdwp::agent::error_code::JdwpError;
use crate::jdwp::agent::packet::{CommandPacket, Packet, ReplyPacket};
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread::JoinHandle;

pub mod command;
pub mod error_code;
pub mod packet;
pub mod reader;

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
    let cmd = JdwpCommand::parse(
        cmd_packet.command_set,
        cmd_packet.command,
        &cmd_packet.data,
        debug.clone(),
    )?;

    println!("Received command: {:?}", cmd);

    let data = match cmd {
        JdwpCommand::VmIdSizes => Ok(handle_id_size()),
        JdwpCommand::EventRequestSet(event_request) => {
            let event_id = event_request.id;
            debug.add_event_request(event_request);
            Ok(event_id.to_be_bytes().to_vec())
        }
        JdwpCommand::VmCapabilities => Ok(handle_vm_capabilities()),
        JdwpCommand::VmCapabilitiesNew => Ok(handle_vm_capabilities_new()),
        JdwpCommand::VmVersion => Ok(handle_vm_version()),
        cmd => {
            eprintln!("Unhandled command: {:?}", cmd);
            Err(JdwpError::ConnectionClosed)
        }
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

fn handle_vm_version() -> Vec<u8> {
    let description = "Java Debug Wire Protocol (Reference Implementation)";
    let jdwp_major: i32 = 25;
    let jdwp_minor: i32 = 0;
    let vm_version = "25.0";
    let vm_name = "toyjvm";

    let mut buf = Vec::new();

    buf.extend(&(description.len() as i32).to_be_bytes());
    buf.extend(description.as_bytes());
    buf.extend(&jdwp_major.to_be_bytes());
    buf.extend(&jdwp_minor.to_be_bytes());
    buf.extend(&(vm_version.len() as i32).to_be_bytes());
    buf.extend(vm_version.as_bytes());
    buf.extend(&(vm_name.len() as i32).to_be_bytes());
    buf.extend(vm_name.as_bytes());

    buf
}

fn handle_vm_capabilities() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend(&0u8.to_be_bytes()); // canWatchFieldModification
    buf.extend(&0u8.to_be_bytes()); // canWatchFieldAccess
    buf.extend(&0u8.to_be_bytes()); // canGetBytecodes
    buf.extend(&0u8.to_be_bytes()); // canGetSyntheticAttribute
    buf.extend(&0u8.to_be_bytes()); // canGetOwnedMonitorInfo
    buf.extend(&0u8.to_be_bytes()); // canGetCurrentContendedMonitor
    buf.extend(&0u8.to_be_bytes()); // canGetMonitorInfo

    buf
}

fn handle_vm_capabilities_new() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend(&0u8.to_be_bytes()); // canWatchFieldModification
    buf.extend(&0u8.to_be_bytes()); // canWatchFieldAccess
    buf.extend(&0u8.to_be_bytes()); // canGetBytecodes
    buf.extend(&0u8.to_be_bytes()); // canGetSyntheticAttribute
    buf.extend(&0u8.to_be_bytes()); // canGetOwnedMonitorInfo
    buf.extend(&0u8.to_be_bytes()); // canGetCurrentContendedMonitor
    buf.extend(&0u8.to_be_bytes()); // canGetMonitorInfo
    buf.extend(&0u8.to_be_bytes()); // canRedefineClasses
    buf.extend(&0u8.to_be_bytes()); // canAddMethod
    buf.extend(&0u8.to_be_bytes()); // canUnrestrictedlyRedefineClasses
    buf.extend(&0u8.to_be_bytes()); // canPopFrames
    buf.extend(&0u8.to_be_bytes()); // canUseInstanceFilters
    buf.extend(&0u8.to_be_bytes()); // canGetSourceDebugExtension
    buf.extend(&0u8.to_be_bytes()); // canRequestVMDeathEvent
    buf.extend(&0u8.to_be_bytes()); // canSetDefaultStratum
    buf.extend(&0u8.to_be_bytes()); // canGetInstanceInfo
    buf.extend(&0u8.to_be_bytes()); // canRequestMonitorEvents
    buf.extend(&0u8.to_be_bytes()); // canGetMonitorFrameInfo
    buf.extend(&0u8.to_be_bytes()); // canUseSourceNameFilters
    buf.extend(&0u8.to_be_bytes()); // canGetConstantPool
    buf.extend(&0u8.to_be_bytes()); // canForceEarlyReturn
    buf.extend(&0u8.to_be_bytes()); // reserved22
    buf.extend(&0u8.to_be_bytes()); // reserved23
    buf.extend(&0u8.to_be_bytes()); // reserved24
    buf.extend(&0u8.to_be_bytes()); // reserved25
    buf.extend(&0u8.to_be_bytes()); // reserved26
    buf.extend(&0u8.to_be_bytes()); // reserved27
    buf.extend(&0u8.to_be_bytes()); // reserved28
    buf.extend(&0u8.to_be_bytes()); // reserved29
    buf.extend(&0u8.to_be_bytes()); // reserved30
    buf.extend(&0u8.to_be_bytes()); // reserved31
    buf.extend(&0u8.to_be_bytes()); // reserved32

    buf
}

fn handle_vm_all_classes() -> Vec<u8> {
    let mut buf = Vec::new();
    buf
}
