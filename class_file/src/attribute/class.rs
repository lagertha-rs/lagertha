use crate::constant_pool::ConstantPool;
use crate::ClassFileErr;
use common::cursor::ByteCursor;
use std::fmt;
use std::fmt::{Display, Formatter};

pub const ATTR_SOURCE_FILE: &[u8] = b"SourceFile";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassAttribute {
    SourceFile { sourcefile_index: u16 },
    Unknown { name_index: u16, info: Vec<u8> },
}

impl<'a> ClassAttribute {
    pub(crate) fn read(
        pool: &ConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let attribute_length = cursor.u32()? as usize;

        let utf8 = pool.get_utf8(attribute_name_index)?.as_bytes();
        match utf8 {
            ATTR_SOURCE_FILE => Ok(ClassAttribute::SourceFile {
                sourcefile_index: cursor.u16()?,
            }),
            _ => {
                let mut buf = vec![0u8; attribute_length];
                cursor.read_exact(&mut buf)?;
                Ok(ClassAttribute::Unknown {
                    name_index: attribute_name_index,
                    info: buf,
                })
            }
        }
    }
}

impl Display for ClassAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClassAttribute::SourceFile { sourcefile_index } => {
                write!(f, "SourceFile(sourcefile_index: {})", sourcefile_index)
            }
            ClassAttribute::Unknown { name_index, info } => write!(
                f,
                "Unsupported(name_index: {}, data: {} bytes)",
                name_index,
                info.len()
            ),
        }
    }
}
