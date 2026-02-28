use crate::flags::ClassFlags;

impl ClassFlags {
    pub fn set_public(&mut self) {
        self.0 |= 0x0001;
    }

    pub fn set_final(&mut self) {
        self.0 |= 0x0010;
    }

    pub fn set_super(&mut self) {
        self.0 |= 0x0020;
    }

    pub fn set_interface(&mut self) {
        self.0 |= 0x0200;
    }

    pub fn set_abstract(&mut self) {
        self.0 |= 0x0400;
    }

    pub fn set_synthetic(&mut self) {
        self.0 |= 0x1000;
    }

    pub fn set_annotation(&mut self) {
        self.0 |= 0x2000;
    }

    pub fn set_enum(&mut self) {
        self.0 |= 0x4000;
    }

    pub fn set_module(&mut self) {
        self.0 |= 0x8000;
    }
}
