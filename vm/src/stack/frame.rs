use std::rc::Rc;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.6
pub struct Frame {
    locals: Vec<()>,
    operands: Vec<()>,
    cp: Rc<()>,
}
