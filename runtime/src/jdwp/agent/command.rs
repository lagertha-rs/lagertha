use crate::jdwp::agent::error_code::JdwpError;
use crate::jdwp::{DebugState, EventKind, EventRequestId, SuspendPolicy};
use byteorder::{BigEndian, ReadBytesExt};
use itertools::Itertools;
use std::io::{Cursor, Read};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct EventRequest {
    pub id: EventRequestId,
    pub event_kind: EventKind,
    pub suspend_policy: SuspendPolicy,
    pub modifiers: Vec<EventModifier>,
}

#[derive(Debug, Clone)]
pub enum EventModifier {
    PlatformThreadsOnly,
    ClassMatch { class_pattern: String },
}

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

    // ReferenceType (2)
    ReferenceTypeInterfaces { class_id: u32 },

    // ClassType (3)
    ClassTypeSuperclass { class_id: u32 },

    // EventRequest (15)
    EventRequestSet(EventRequest),
    EventRequestClear,
    EventRequestClearAllBreakpoints,
}

impl JdwpCommand {
    pub fn parse(
        command_set: u8,
        command: u8,
        data: &[u8],
        state: Arc<DebugState>,
    ) -> Result<Self, JdwpError> {
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

            // ReferenceType
            (2, 10) => Ok(JdwpCommand::ReferenceTypeInterfaces {
                class_id: cursor.read_u32::<BigEndian>()?,
            }),

            // ClassType
            (3, 1) => Ok(JdwpCommand::ClassTypeSuperclass {
                class_id: cursor.read_u32::<BigEndian>()?,
            }),

            // EventRequest
            (15, 1) => {
                let event_kind = cursor.read_u8()?;
                let suspend_policy = cursor.read_u8()?;
                let modifier_count = cursor.read_i32::<BigEndian>()?;

                let mut modifiers = Vec::with_capacity(modifier_count as usize);
                if modifier_count > 0 {
                    let kind = cursor.read_u8()?;
                    match kind {
                        5 => {
                            let class_pattern = {
                                let str_len = cursor.read_i32::<BigEndian>()? as usize;
                                let mut str_bytes = vec![0u8; str_len];
                                cursor.read_exact(&mut str_bytes)?;
                                for i in 0..str_len {
                                    if str_bytes[i] == b'.' {
                                        str_bytes[i] = b'/';
                                    }
                                }
                                String::from_utf8(str_bytes).unwrap()
                            };
                            modifiers.push(EventModifier::ClassMatch { class_pattern });
                        }
                        13 => {
                            modifiers.push(EventModifier::PlatformThreadsOnly);
                        }
                        other => {
                            unimplemented!("Event modifier kind {} not implemented", other);
                        }
                    }
                }
                let id = state.get_next_event_id();
                Ok(JdwpCommand::EventRequestSet(EventRequest {
                    id,
                    event_kind: EventKind::try_from(event_kind).unwrap(),
                    suspend_policy: SuspendPolicy::try_from(suspend_policy).unwrap(),
                    modifiers,
                }))
            }
            (15, 2) => todo!(),
            (15, 3) => Ok(JdwpCommand::EventRequestClearAllBreakpoints),
            _ => Err(JdwpError::UnknownCommand {
                command_set,
                command,
            }),
        }
    }
}
