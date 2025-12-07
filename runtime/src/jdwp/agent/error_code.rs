use std::io;

pub const NONE: u16 = 0;
pub const INVALID_THREAD: u16 = 10;
pub const INVALID_THREAD_GROUP: u16 = 11;
pub const INVALID_PRIORITY: u16 = 12;
pub const THREAD_NOT_SUSPENDED: u16 = 13;
pub const THREAD_SUSPENDED: u16 = 14;
pub const THREAD_NOT_ALIVE: u16 = 15;
pub const INVALID_OBJECT: u16 = 20;
pub const INVALID_CLASS: u16 = 21;
pub const CLASS_NOT_PREPARED: u16 = 22;
pub const INVALID_METHODID: u16 = 23;
pub const INVALID_LOCATION: u16 = 24;
pub const INVALID_FIELDID: u16 = 25;
pub const INVALID_FRAMEID: u16 = 30;
pub const NO_MORE_FRAMES: u16 = 31;
pub const OPAQUE_FRAME: u16 = 32;
pub const NOT_CURRENT_FRAME: u16 = 33;
pub const TYPE_MISMATCH: u16 = 34;
pub const INVALID_SLOT: u16 = 35;
pub const DUPLICATE: u16 = 40;
pub const NOT_FOUND: u16 = 41;
pub const INVALID_MODULE: u16 = 42;
pub const INVALID_MONITOR: u16 = 50;
pub const NOT_MONITOR_OWNER: u16 = 51;
pub const INTERRUPT: u16 = 52;
pub const INVALID_CLASS_FORMAT: u16 = 60;
pub const CIRCULAR_CLASS_DEFINITION: u16 = 61;
pub const FAILS_VERIFICATION: u16 = 62;
pub const ADD_METHOD_NOT_IMPLEMENTED: u16 = 63;
pub const SCHEMA_CHANGE_NOT_IMPLEMENTED: u16 = 64;
pub const INVALID_TYPESTATE: u16 = 65;
pub const HIERARCHY_CHANGE_NOT_IMPLEMENTED: u16 = 66;
pub const DELETE_METHOD_NOT_IMPLEMENTED: u16 = 67;
pub const UNSUPPORTED_VERSION: u16 = 68;
pub const NAMES_DONT_MATCH: u16 = 69;
pub const CLASS_MODIFIERS_CHANGE_NOT_IMPLEMENTED: u16 = 70;
pub const METHOD_MODIFIERS_CHANGE_NOT_IMPLEMENTED: u16 = 71;
pub const NOT_IMPLEMENTED: u16 = 99;
pub const NULL_POINTER: u16 = 100;
pub const ABSENT_INFORMATION: u16 = 101;
pub const INVALID_EVENT_TYPE: u16 = 102;
pub const ILLEGAL_ARGUMENT: u16 = 103;
pub const OUT_OF_MEMORY: u16 = 110;
pub const ACCESS_DENIED: u16 = 111;
pub const VM_DEAD: u16 = 112;
pub const INTERNAL: u16 = 113;
pub const UNATTACHED_THREAD: u16 = 115;
pub const INVALID_TAG: u16 = 500;
pub const ALREADY_INVOKING: u16 = 502;
pub const INVALID_INDEX: u16 = 503;
pub const INVALID_LENGTH: u16 = 504;
pub const INVALID_STRING: u16 = 506;
pub const INVALID_CLASS_LOADER: u16 = 507;
pub const INVALID_ARRAY: u16 = 508;
pub const TRANSPORT_LOAD: u16 = 509;
pub const TRANSPORT_INIT: u16 = 510;
pub const NATIVE_METHOD: u16 = 511;
pub const INVALID_COUNT: u16 = 512;

#[derive(Debug)]
pub enum JdwpError {
    Io(io::Error),
    InvalidPacketLength(u32),
    InvalidFlags(u8),
    ConnectionClosed,
    UnknownCommand { command_set: u8, command: u8 },
}

impl From<io::Error> for JdwpError {
    fn from(e: io::Error) -> Self {
        if e.kind() == io::ErrorKind::UnexpectedEof {
            JdwpError::ConnectionClosed
        } else {
            JdwpError::Io(e)
        }
    }
}

impl std::fmt::Display for JdwpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JdwpError::Io(e) => write!(f, "IO error: {}", e),
            JdwpError::InvalidPacketLength(len) => write!(f, "Invalid packet length: {}", len),
            JdwpError::InvalidFlags(flags) => write!(f, "Invalid flags: 0x{:02x}", flags),
            JdwpError::ConnectionClosed => write!(f, "Connection closed"),
            JdwpError::UnknownCommand {
                command_set,
                command,
            } => {
                write!(
                    f,
                    "Unknown command: set={}, command={}",
                    command_set, command
                )
            }
        }
    }
}

impl std::error::Error for JdwpError {}
