use crate::flags::ClassFlags;

impl ClassFlags {
    pub fn set_public(&mut self) {
        self.0 |= 0x0001;
    }
}
