use crate::flags::ClassFlags;

impl ClassFlags {
    pub fn set_public(&mut self) {
        self.0 |= 0x0001;
    }

    pub fn set_final(&mut self) {
        self.0 |= 0x0010;
    }
}
