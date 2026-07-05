use crate::ClassFile;
use crate::flags::ClassFlags;

pub enum Finding {
    ClassFlag(ClassFlagsFinding),
}

pub enum ClassFlagsFinding {
    InterfaceWithoutAbstract(ClassFlags),
    InterfaceIncompatibleFlags(ClassFlags),
}

impl ClassFlagsFinding {
    pub fn get_flags(&self) -> ClassFlags {
        match self {
            ClassFlagsFinding::InterfaceWithoutAbstract(flags)
            | ClassFlagsFinding::InterfaceIncompatibleFlags(flags) => *flags,
        }
    }
}

impl ClassFile {
    pub fn verify(&self) -> Vec<Finding> {
        let mut findings = Vec::new();
        self.verify_interface(&mut findings);
        findings
    }

    fn verify_interface(&self, findings: &mut Vec<Finding>) {
        let f = self.access_flags;
        if !f.is_interface() {
            return;
        }
        if !f.is_abstract() {
            findings.push(Finding::ClassFlag(
                ClassFlagsFinding::InterfaceWithoutAbstract(f),
            ));
        }
        if f.is_final() || f.is_super() || f.is_enum() || f.is_module() {
            // TODO: don't use magic consts
            let incompatible = ClassFlags::new(*f.get_raw() & (0x0010 | 0x0020 | 0x4000 | 0x8000));
            findings.push(Finding::ClassFlag(
                ClassFlagsFinding::InterfaceIncompatibleFlags(incompatible),
            ));
        }
    }
}
