use crate::class_file::ClassFile;
use crate::rt::class::access::ClassAccessFlag;
use crate::rt::class::field::Field;
use crate::rt::class::method::Method;
use crate::rt::constant_pool::reference::ClassReference;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::JvmError;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Class {
    this: Rc<ClassReference>,
    access: ClassAccessFlag,
    minor_version: u16,
    major_version: u16,
    super_class: Option<Rc<ClassReference>>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    interfaces: Vec<String>,
    attributes: Vec<String>,
    cp: Rc<RuntimeConstantPool>,
    initialized: bool,
}

impl Class {
    pub fn new(cf: ClassFile) -> Result<Self, JvmError> {
        let cp = Rc::new(RuntimeConstantPool::new(cf.constant_pool));
        let minor_version = cf.minor_version;
        let major_version = cf.major_version;
        let this = cp.get_class(&cf.this_class)?.clone();
        let super_class = if cf.super_class != 0 {
            Some(cp.get_class(&cf.super_class)?.clone())
        } else {
            None
        };
        let access = ClassAccessFlag::new(cf.access_flags);
        let methods = cf
            .methods
            .iter()
            .map(|method| Method::new(method, cp.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Class {
            this,
            access,
            super_class,
            minor_version,
            major_version,
            fields: vec![],
            methods,
            interfaces: vec![],
            attributes: vec![],
            cp,
            initialized: false,
        })
    }

    pub fn get_name(&self) -> Result<&Rc<String>, JvmError> {
        self.this.name().map_err(Into::into)
    }
}

// TODO: all nested displays have hardcoded space count on the beginning, find elegant solution
// TODO: Seems my was of error of mapping to fmt error skips the error message, hard to debug
// TODO: clean up all nested displays
impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} class {}",
            self.access,
            self.this
                .name()
                .map(|v| v.replace("/", "."))
                .map_err(Into::into)?
        )?;
        writeln!(f, "  minor version: {}", self.minor_version)?;
        writeln!(f, "  major version: {}", self.major_version)?;
        writeln!(
            f,
            "  flags: (0x{:04X}) {}",
            self.access.get_raw(),
            self.access
        )?;
        writeln!(
            f,
            "  this_class: #{}\t\t// {}",
            self.this.name_index(),
            self.this.name().map_err(Into::into)?
        )?;
        if let Some(super_class) = &self.super_class {
            writeln!(
                f,
                "  super_class: #{}\t\t// {}",
                super_class.name_index(),
                super_class.name().map_err(Into::into)?
            )?;
        } else {
            writeln!(f, "  super_class: #0",)?;
        }
        writeln!(
            f,
            "  interfaces: {}, fields: {}, methods: {}, attributes: {}",
            self.interfaces.len(),
            self.fields.len(),
            self.methods.len(),
            self.attributes.len()
        )?;
        writeln!(f, "Constant pool:\n{}", self.cp)?;
        writeln!(f, "{{")?;
        for method in &self.methods {
            writeln!(f, "{method}")?;
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}
