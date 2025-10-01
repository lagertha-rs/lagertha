use crate::attribute::class::ClassAttribute;
use crate::constant::pool::ConstantPool;
use crate::error::ClassFormatErr;
use crate::flags::ClassFlags;
use common::utils::cursor::ByteCursor;
use constant::ConstantInfo;
use field::FieldInfo;
use method::MethodInfo;

pub mod attribute;
pub mod constant;
pub mod error;
pub mod field;
pub mod flags;
pub mod method;

#[cfg(feature = "pretty_print")]
pub mod print;
// TODO: review all access levels in the crate (methods, fields, modules, structs, etc.)
// TODO: align enums that end with "Info"/"Ref" and "Type"/"Kind" suffixes

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html
/// A rust representation of a Java .class file. All structures in the crates have public only public
/// fields for easier access, because anyway it will be remapped to runtime structures.
///
/// All print related code is behind the `pretty_print` feature flag.
#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub cp: ConstantPool,
    pub access_flags: ClassFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<ClassAttribute>,
}

impl ClassFile {
    const MAGIC: u32 = 0xCAFEBABE;
    fn validate_magic(val: u32) -> Result<(), ClassFormatErr> {
        (val == ClassFile::MAGIC)
            .then_some(())
            .ok_or(ClassFormatErr::WrongMagic(val))
    }
}

impl TryFrom<Vec<u8>> for ClassFile {
    type Error = ClassFormatErr;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = ByteCursor::new(&value);
        let magic = cursor.u32()?;
        ClassFile::validate_magic(magic)?;
        let minor_version = cursor.u16()?;
        let major_version = cursor.u16()?;
        let constant_pool_count = cursor.u16()?;
        let mut constant_pool = Vec::with_capacity((constant_pool_count + 1) as usize);
        constant_pool.push(ConstantInfo::Unused);
        let mut i = 1;
        while i < constant_pool_count {
            let constant = ConstantInfo::read(&mut cursor)?;
            constant_pool.push(constant.clone());
            match constant {
                // described in JVM spec that Long and Double take two entries in the constant pool
                // https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.4.5
                ConstantInfo::Long(_) | ConstantInfo::Double(_) => {
                    constant_pool.push(ConstantInfo::Unused);
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            }
        }
        let constant_pool = ConstantPool {
            inner: constant_pool,
        };

        let access_flags = ClassFlags::new(cursor.u16()?);
        let this_class = cursor.u16()?;
        let super_class = cursor.u16()?;
        let interfaces_count = cursor.u16()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(cursor.u16()?);
        }
        let fields_count = cursor.u16()?;
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(FieldInfo::read(&constant_pool, &mut cursor)?);
        }
        let methods_count = cursor.u16()?;
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(MethodInfo::read(&constant_pool, &mut cursor)?);
        }
        let attributes_count = cursor.u16()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(ClassAttribute::read(&constant_pool, &mut cursor)?);
        }

        if cursor.u8().is_ok() {
            Err(ClassFormatErr::TrailingBytes)
        } else {
            Ok(Self {
                minor_version,
                major_version,
                cp: constant_pool,
                access_flags,
                this_class,
                super_class,
                interfaces,
                fields,
                methods,
                attributes,
            })
        }
    }
}

#[cfg(feature = "pretty_print")]
impl ClassFile {
    const COMMENT_WIDTH: usize = 24;
    const CONSTANT_KIND_WIDTH: usize = 18;

    fn fmt_generic_signature(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use crate::attribute::SharedAttribute;
        use common::pretty_try;
        use common::signature::ClassSignature;
        use std::fmt::Write as _;

        // for java.lang.Object
        if self.super_class == 0 {
            return Ok(());
        }
        let super_class_name = pretty_try!(ind, self.cp.get_pretty_class_name(&self.super_class));
        let super_is_object = super_class_name == "java.lang.Object";
        if let Some(sig_index) = self.attributes.iter().find_map(|attr| {
            if let ClassAttribute::Shared(shared) = attr {
                match shared {
                    SharedAttribute::Signature(sig_index) => Some(sig_index),
                    _ => None,
                }
            } else {
                None
            }
        }) {
            let raw_sig = pretty_try!(ind, self.cp.get_utf8(sig_index));
            let sig = pretty_try!(ind, ClassSignature::try_from(raw_sig));
            write!(ind, "{}", sig)
        } else if !super_is_object || !self.interfaces.is_empty() {
            let interfaces_names = pretty_try!(
                ind,
                self.interfaces
                    .iter()
                    .map(|i| self.cp.get_pretty_class_name(i))
                    .collect::<Result<Vec<_>, _>>()
            );

            if !super_is_object {
                write!(ind, "extends {}", super_class_name)?;
                if !interfaces_names.is_empty() {
                    write!(ind, " ")?;
                }
            }
            if !interfaces_names.is_empty() {
                write!(ind, "implements {}", interfaces_names.join(", "))?;
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    fn fmt_java_like_signature(
        &self,
        ind: &mut common::utils::indent_write::Indented<'_>,
    ) -> std::fmt::Result {
        use common::pretty_class_name_try;
        use std::fmt::Write as _;

        self.access_flags.fmt_pretty_java_like_prefix(ind)?;
        write!(
            ind,
            "{} ",
            pretty_class_name_try!(ind, self.cp.get_class_name(&self.this_class))
        )?;
        self.fmt_generic_signature(ind)?;
        writeln!(ind)
    }
}

#[cfg(feature = "pretty_print")]
impl std::fmt::Display for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use common::pretty_try;
        use common::utils::indent_write::Indented;
        use std::fmt::Write as _;

        let mut ind = Indented::new(f);

        self.fmt_java_like_signature(&mut ind)?;

        ind.with_indent(|ind| {
            writeln!(ind, "minor version: {}", self.minor_version)?;
            writeln!(ind, "major version: {}", self.major_version)?;

            write!(ind, "flags: (0x{:04X}) ", self.access_flags.get_raw(),)?;
            self.access_flags.fmt_class_javap_like_list(ind)?;
            writeln!(ind)?;

            writeln!(
                ind,
                "this_class: {:<w$} //{}",
                format!("#{}", self.this_class),
                pretty_try!(ind, self.cp.get_class_name(&self.this_class)),
                w = Self::COMMENT_WIDTH
            )?;
            write!(
                ind,
                "super_class: {:<w$}",
                format!("#{}", self.super_class),
                w = Self::COMMENT_WIDTH
            )?;
            if self.super_class != 0 {
                write!(
                    ind,
                    "//{}",
                    pretty_try!(ind, self.cp.get_class_name(&self.super_class))
                )?;
            }
            writeln!(ind)?;
            writeln!(
                ind,
                "interfaces: {}, fields: {}, methods: {}, attributes: {}",
                self.interfaces.len(),
                self.fields.len(),
                self.methods.len(),
                self.attributes.len()
            )?;
            Ok(())
        })?;
        writeln!(ind, "Constant pool:")?;
        ind.with_indent(|ind| {
            let counter_width = self
                .cp
                .inner
                .len()
                .checked_ilog10()
                .map_or(0, |d| d as usize)
                + 2;
            for (i, c) in self.cp.inner.iter().enumerate() {
                if matches!(c, ConstantInfo::Unused) {
                    continue;
                }
                let tag = format_args!("{:<kw$}", c.get_tag(), kw = Self::CONSTANT_KIND_WIDTH);
                write!(ind, "{:>w$} = {} ", format!("#{i}"), tag, w = counter_width)?;
                c.fmt_pretty(ind, &self.cp)?;
            }
            Ok(())
        })?;
        if !self.fields.is_empty() || !self.methods.is_empty() {
            writeln!(ind, "{{")?;
            ind.with_indent(|ind| {
                for (i, field) in self.fields.iter().enumerate() {
                    field.fmt_pretty(ind, &self.cp)?;
                    if i + 1 < self.fields.len() {
                        writeln!(ind)?;
                    }
                }
                if !self.fields.is_empty() {
                    writeln!(ind)?;
                }

                for (i, method) in self.methods.iter().enumerate() {
                    method.fmt_pretty(ind, &self.cp, &self.this_class)?;
                    if i + 1 < self.methods.len() {
                        writeln!(ind)?;
                    }
                }
                Ok(())
            })?;
            writeln!(ind, "}}")?;
        }
        for attribute in &self.attributes {
            attribute.fmt_pretty(&mut ind, &self.cp)?;
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "pretty_print"))]
mod tests {
    use super::*;
    use insta::with_settings;
    use rstest::*;
    use std::fs;
    use std::io::BufRead;
    use std::path::{Path, PathBuf};

    const DISPLAY_SNAPSHOT_PATH: &str = "../snapshots";

    fn to_snapshot_name(path: &Path) -> String {
        let mut iter = path.iter().map(|s| s.to_string_lossy().to_string());
        for seg in iter.by_ref() {
            if seg == "test-classes" {
                break;
            }
        }
        let tail: Vec<String> = iter.collect();
        tail.join("-")
    }

    #[rstest]
    #[trace]
    // Requires `testdata/compile-fixtures.sh` to be run to generate the .class files
    fn test_display(
        #[base_dir = "../target/test-classes/"]
        #[files("**/*.class")]
        path: PathBuf,
    ) {
        // Given
        let bytes = fs::read(&path).unwrap_or_else(|_| panic!("Can't read file {:?}", path));

        // When
        let display = format!("{}", ClassFile::try_from(bytes).unwrap());

        // Then
        with_settings!(
            {
                snapshot_path => DISPLAY_SNAPSHOT_PATH,
                prepend_module_to_snapshot => false,
            },
            {
                insta::assert_snapshot!(to_snapshot_name(&path), display);
            }
        );
    }

    #[rstest]
    // Requires `testdata/compile-fixtures.sh` to be run to generate the .class files
    fn compare_with_javap(
        #[base_dir = "../target/test-classes"]
        #[files("**/*.class")]
        path: PathBuf,
    ) {
        // Given
        let bytes = fs::read(&path).unwrap_or_else(|_| panic!("Can't read file {:?}", path));
        let my_display = format!("{}", ClassFile::try_from(bytes).unwrap())
            .as_bytes()
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let javap = std::process::Command::new("javap")
            .arg("-v")
            .arg("-p")
            .arg(&path)
            .output()
            .unwrap_or_else(|e| panic!("Can't run javap on file {:?}:{}\n", path, e))
            .stdout
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let javap_display = javap[4..javap.len()].to_vec();

        // When && Then
        for (i, (my, javap)) in my_display.iter().zip(javap_display.iter()).enumerate() {
            let my_line_normalized: String = my.chars().filter(|c| !c.is_whitespace()).collect();
            let javap_line_normalized: String =
                javap.chars().filter(|c| !c.is_whitespace()).collect();
            assert_eq!(
                my_line_normalized,
                javap_line_normalized,
                "Mismatch at line {} of file {:?}. My line: {:?}, javap line: {:?}",
                i + 1,
                path,
                my,
                javap
            );
        }
    }
}
