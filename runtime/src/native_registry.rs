use crate::Value;
use std::collections::HashMap;

/* TODO: good idea from chatgpt. as I can't use JDK classes right now, as a stub for the methods in
    external classes, I can treat it as "native" and define by myself
*/

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct MethodKey {
    pub class: String,
    pub name: String,
    pub desc: String,
}

pub type NativeFn = fn(args: &[Value]) -> Option<Value>;

pub struct NativeRegistry {
    map: HashMap<MethodKey, NativeFn>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: MethodKey, f: NativeFn) {
        self.map.insert(key, f);
    }

    pub fn get(&self, key: &MethodKey) -> Option<&NativeFn> {
        self.map.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    fn fake_println(args: &[Value]) -> Option<Value> {
        assert_eq!(args.len(), 1);
        println!("(fake println) {:?}", args[0]);
        None
    }

    #[test]
    fn register_and_call() {
        let mut reg = NativeRegistry::new();
        let key = MethodKey {
            class: "java/io/PrintStream".into(),
            name: "println".into(),
            desc: "(Ljava/lang/String;)V".into(),
        };
        reg.register(key.clone(), fake_println);
        let f = reg.get(&key).unwrap();
        f(&[Value::Null]);
    }
}
