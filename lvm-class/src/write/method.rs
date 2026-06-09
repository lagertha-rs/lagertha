use crate::constant_pool::ConstantPool;
use crate::member::MethodInfo;

impl MethodInfo {
    pub fn write_to(&self, buf: &mut Vec<u8>, cp: &ConstantPool) {
        buf.extend_from_slice(&self.access_flags.get_raw().to_be_bytes());
        buf.extend_from_slice(&self.name_index.to_be_bytes());
        buf.extend_from_slice(&self.descriptor_index.to_be_bytes());
        buf.extend_from_slice(&(self.attributes.len() as u16).to_be_bytes());
        for attr in &self.attributes {
            attr.write_to(buf, cp);
        }
    }
}
