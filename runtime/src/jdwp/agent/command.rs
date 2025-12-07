use crate::jdwp::agent::error_code::JdwpError;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub enum JdwpCommand {
    // VirtualMachine (1)
    VmVersion,
    VmClassesBySignature { signature: String },
    VmAllClasses,
    VmAllThreads,
    VmTopLevelThreadGroups,
    VmDispose,
    VmIdSizes,
    VmSuspend,
    VmResume,
    VmExit { exit_code: i32 },
    VmCreateString { utf: String },
    VmCapabilities,
    VmClassPaths,
    VmDisposeObjects { objects: Vec<u64> },
    VmHoldEvents,
    VmReleaseEvents,
    VmCapabilitiesNew,
    VmRedefineClasses,
    VmSetDefaultStratum { stratum: String },
    VmAllClassesWithGeneric,
    VmInstanceCounts { ref_types: Vec<u64> },
    VmAllModules,

    // EventRequest (15)
    EventRequestSet {},
    EventRequestClear,
    EventRequestClearAllBreakpoints,
}

impl JdwpCommand {
    pub fn parse(command_set: u8, command: u8, data: &[u8]) -> Result<Self, JdwpError> {
        let mut cursor = Cursor::new(data);

        match (command_set, command) {
            // VirtualMachine
            (1, 1) => Ok(JdwpCommand::VmVersion),
            (1, 2) => todo!(),
            (1, 3) => Ok(JdwpCommand::VmAllClasses),
            (1, 4) => Ok(JdwpCommand::VmAllThreads),
            (1, 5) => Ok(JdwpCommand::VmTopLevelThreadGroups),
            (1, 6) => Ok(JdwpCommand::VmDispose),
            (1, 7) => Ok(JdwpCommand::VmIdSizes),
            (1, 8) => Ok(JdwpCommand::VmSuspend),
            (1, 9) => Ok(JdwpCommand::VmResume),
            (1, 10) => Ok(JdwpCommand::VmExit {
                exit_code: cursor.read_i32::<BigEndian>()?,
            }),
            (1, 11) => todo!(),
            (1, 12) => Ok(JdwpCommand::VmCapabilities),
            (1, 13) => Ok(JdwpCommand::VmClassPaths),
            (1, 14) => todo!(),
            (1, 15) => Ok(JdwpCommand::VmHoldEvents),
            (1, 16) => Ok(JdwpCommand::VmReleaseEvents),
            (1, 17) => Ok(JdwpCommand::VmCapabilitiesNew),
            (1, 18) => todo!(),
            (1, 19) => todo!(),
            (1, 20) => Ok(JdwpCommand::VmAllClassesWithGeneric),
            (1, 21) => todo!(),
            (1, 22) => Ok(JdwpCommand::VmAllModules),

            // EventRequest
            (15, 1) => todo!(),
            (15, 2) => todo!(),
            (15, 3) => Ok(JdwpCommand::EventRequestClearAllBreakpoints),
            _ => Err(JdwpError::UnknownCommand {
                command_set,
                command,
            }),
        }
    }
}
