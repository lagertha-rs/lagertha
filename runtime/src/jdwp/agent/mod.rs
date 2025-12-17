use crate::VirtualMachine;
use crate::jdwp::agent::command::JdwpCommand;
use crate::jdwp::agent::error_code::JdwpError;
use crate::jdwp::agent::packet::{CommandPacket, Packet, ReplyPacket};
use crate::jdwp::{DebugEvent, DebugState};
use crate::keys::ClassId;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::thread::JoinHandle;

pub mod command;
pub mod error_code;
pub mod packet;
pub mod reader;

const HANDSHAKE: &[u8; 14] = b"JDWP-Handshake";

pub fn start_jdwp_agent(
    vm: Arc<VirtualMachine>,
    debug: Arc<DebugState>,
    port: u16,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        jdwp_agent_routine(vm, debug, port);
    })
}

fn jdwp_agent_routine(vm: Arc<VirtualMachine>, debug: Arc<DebugState>, port: u16) {
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

        handle_connection(vm.clone(), debug.clone(), stream);

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

fn send_events(stream: &mut TcpStream, events: &[DebugEvent]) -> Result<(), JdwpError> {
    let mut buffer = Vec::new();
    buffer.extend(&2u8.to_be_bytes()); // TODO: hardcoded suspend policy: SUSPEND_ALL
    buffer.extend(&(events.len() as u32).to_be_bytes()); // number of events
    for event in events {
        match event {
            DebugEvent::VMStart => {
                buffer.extend(90u8.to_be_bytes()); // TODO: hardcoded event kind: VM_START
                buffer.extend(&0u32.to_be_bytes()); // request id
                buffer.extend(&1u32.to_be_bytes()); // TODO: hardcoded thread id: 1
            }
            DebugEvent::VMDeath => {
                buffer.extend(99u8.to_be_bytes()); // TODO: hardcoded event kind: VM_DEATH
                buffer.extend(&0u32.to_be_bytes()); // request id
            }
            DebugEvent::ClassPrepare(info) => {
                buffer.extend(8u8.to_be_bytes()); // TODO: hardcoded event kind: CLASS_PREPARE
                buffer.extend(&0u32.to_be_bytes()); // request id
                buffer.extend(&info.thread_id.to_be_bytes()); // thread id
                buffer.extend(&(info.ref_type_tag as u8).to_be_bytes()); // ref type tag
                buffer.extend(&info.type_id.to_be_bytes()); // class id
                let signature_bytes = info.signature.as_bytes();
                buffer.extend(&(signature_bytes.len() as u32).to_be_bytes());
                buffer.extend(signature_bytes);
                buffer.extend(&(info.status as i32).to_be_bytes()); // status
            }
        }
    }

    let mut header = Vec::new();
    header.extend(&((11 + buffer.len()) as u32).to_be_bytes()); // length
    header.extend(&0u32.to_be_bytes()); // id (0 for events)
    header.extend(&0u8.to_be_bytes()); // flags (0 for command)
    header.extend(&64u8.to_be_bytes()); // command set: Event
    header.extend(&100u8.to_be_bytes()); // command: Composite

    stream.write_all(&header)?;
    stream.write_all(&buffer)?;
    stream.flush()?;

    Ok(())
}

fn handle_connection(vm: Arc<VirtualMachine>, debug: Arc<DebugState>, mut stream: TcpStream) {
    let mut event_buffer = Vec::new();
    loop {
        while let Ok(event) = debug.event_rx.try_recv() {
            event_buffer.push(event)
        }
        if !event_buffer.is_empty() {
            if let Err(e) = send_events(&mut stream, &event_buffer) {
                eprintln!("Error sending events: {}", e);
                break;
            }
            event_buffer.clear();
        }

        match Packet::read(&mut stream) {
            Ok(Packet::Command(cmd_packet)) => match handle_command(&vm, debug.clone(), cmd_packet)
            {
                Ok(None) => {}
                Ok(Some(reply_packet)) => {
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
    vm: &VirtualMachine,
    debug: Arc<DebugState>,
    cmd_packet: CommandPacket,
) -> Result<Option<ReplyPacket>, JdwpError> {
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
            Ok(event_id.0.to_be_bytes().to_vec())
        }
        JdwpCommand::VmResume => {
            debug.resume_all();
            return Ok(None);
        }
        JdwpCommand::VmCapabilities => Ok(handle_vm_capabilities()),
        JdwpCommand::VmCapabilitiesNew => Ok(handle_vm_capabilities_new()),
        JdwpCommand::VmAllClasses => Ok(handle_vm_all_classes(vm)),
        JdwpCommand::VmTopLevelThreadGroups => Ok(handle_top_level_thread_groups()),
        JdwpCommand::VmVersion => Ok(handle_vm_version()),
        JdwpCommand::ClassTypeSuperclass { class_id } => {
            Ok(handle_class_type_superclass(vm, class_id))
        }
        JdwpCommand::ReferenceTypeInterfaces { class_id } => {
            Ok(handle_reference_type_interfaces(vm, class_id))
        }
        cmd => {
            eprintln!("Unhandled command: {:?}", cmd);
            Err(JdwpError::ConnectionClosed)
        }
    }?;

    Ok(Some(ReplyPacket {
        id: cmd_packet.id,
        error_code: 0,
        data,
    }))
}

fn handle_reference_type_interfaces(vm: &VirtualMachine, class_id: u32) -> Vec<u8> {
    let ma_read = vm.method_area_read();
    let class = ma_read.get_class(&ClassId::new(NonZeroU32::new(class_id).unwrap()));
    let interface = class.get_direct_interfaces().unwrap();
    let mut buf = Vec::with_capacity(4 + interface.len() * 4); // number of interfaces + interfaces data
    buf.extend(&(interface.len() as i32).to_be_bytes()); // number of interfaces
    for interface_id in interface {
        buf.extend(&interface_id.to_i32().to_be_bytes()); // interface id
    }
    buf
}

fn handle_class_type_superclass(vm: &VirtualMachine, class_id: u32) -> Vec<u8> {
    let ma_read = vm.method_area_read();
    let class = ma_read.get_class(&ClassId::new(NonZeroU32::new(class_id).unwrap()));
    let super_class_id = if let Some(super_class) = class.get_super_id() {
        super_class.to_i32()
    } else {
        0
    };
    super_class_id.to_be_bytes().to_vec()
}

fn handle_vm_all_classes(vm: &VirtualMachine) -> Vec<u8> {
    let one_class_approx_size = 1 + 4 + 4 + (20 * 8) + 4; // refTypeTag + typeId + signatureLen + signature + status
    let ma_read = vm.method_area_read();
    let classes = ma_read.classes();
    let mut classes_count: i32 = 0;
    let mut buf = Vec::with_capacity(4 + classes.len() * one_class_approx_size); // number of classes + classes data
    buf.extend(&classes_count.to_be_bytes()); // placeholder for number of classes
    //TODO: I guess need to skip primitive types?
    for (i, class) in classes.iter().enumerate() {
        if class.is_primitive() {
            continue;
        }
        classes_count += 1;
        let ref_type_tag: u8 = if class.is_interface() {
            0x02
        } else if class.is_array() {
            0x03
        } else {
            0x01
        };
        buf.extend(&ref_type_tag.to_be_bytes()); // refTypeTag

        let type_id = i as u32 + 1; // typeId
        buf.extend(&type_id.to_be_bytes()); // typeId

        let signature = vm.interner().resolve(&class.get_name());
        buf.extend(&(signature.len() as i32).to_be_bytes()); // signature length
        buf.extend(signature.as_bytes()); // signature

        let status: i32 = 0x02; //TODO: hardcoded status (VERIFIED)
        buf.extend(&status.to_be_bytes()); // status
    }
    // Update the number of classes at the beginning
    let classes_count_bytes = classes_count.to_be_bytes();
    buf[0..4].copy_from_slice(&classes_count_bytes);
    buf
}

fn handle_top_level_thread_groups() -> Vec<u8> {
    let mut response = Vec::new();
    response.extend(&(1i32).to_be_bytes()); // number of thread groups
    response.extend(&(666u32).to_be_bytes()); // TODO: hardcoded thread group id
    response
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
