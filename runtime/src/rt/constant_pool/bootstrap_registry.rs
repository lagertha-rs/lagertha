use crate::{ClassId, FieldKey, MethodKey, Symbol};
use common::error::JvmError;
use lasso::ThreadedRodeo;
use std::cell::OnceCell;

pub struct BootstrapRegistry {
    // Common method keys
    pub clinit_mk: MethodKey,
    pub init_mk: MethodKey,
    pub main_mk: MethodKey,

    // Common field keys
    pub class_name_fk: FieldKey,

    // Common class names (interned)
    pub java_lang_object_sym: Symbol,
    pub java_lang_class_sym: Symbol,
    pub java_lang_string_sym: Symbol,
    pub java_lang_system_sym: Symbol,

    // Common method names (interned)
    pub init_sym: Symbol,
    pub clinit_sym: Symbol,
    pub main_sym: Symbol,

    // Common descriptors (interned)
    pub desc_void_sym: Symbol,         // ()V
    pub desc_string_sym: Symbol,       // Ljava/lang/String;
    pub desc_object_sym: Symbol,       // Ljava/lang/Object;
    pub desc_class_sym: Symbol,        // Ljava/lang/Class;
    pub desc_string_array_sym: Symbol, // [Ljava/lang/String;
    pub desc_char_array_sym: Symbol,   // [C

    // core classes IDs
    java_lang_class_id: OnceCell<ClassId>,
    java_lang_object_id: OnceCell<ClassId>,
}

impl BootstrapRegistry {
    pub fn new(interner: &ThreadedRodeo) -> Self {
        // Method names
        let clinit_sym = interner.get_or_intern("<clinit>");
        let init_sym = interner.get_or_intern("<init>");
        let main_sym = interner.get_or_intern("main");

        // Common descriptors
        let desc_void_sym = interner.get_or_intern("()V");
        let desc_string_sym = interner.get_or_intern("Ljava/lang/String;");
        let desc_object_sym = interner.get_or_intern("Ljava/lang/Object;");
        let desc_class_sym = interner.get_or_intern("Ljava/lang/Class;");
        let desc_string_array_sym = interner.get_or_intern("[Ljava/lang/String;");
        let desc_char_array_sym = interner.get_or_intern("[C");

        // Field names
        let name_field = interner.get_or_intern("name");

        Self {
            // Method keys
            clinit_mk: MethodKey {
                name: clinit_sym,
                desc: desc_void_sym,
            },
            init_mk: MethodKey {
                name: init_sym,
                desc: desc_void_sym,
            },
            main_mk: MethodKey {
                name: main_sym,
                desc: interner.get_or_intern("([Ljava/lang/String;)V"),
            },

            // Field keys
            class_name_fk: FieldKey {
                name: name_field,
                desc: desc_string_sym,
            },

            // Class names
            java_lang_object_sym: interner.get_or_intern("java/lang/Object"),
            java_lang_class_sym: interner.get_or_intern("java/lang/Class"),
            java_lang_string_sym: interner.get_or_intern("java/lang/String"),
            java_lang_system_sym: interner.get_or_intern("java/lang/System"),
            init_sym,
            clinit_sym,
            main_sym,

            // Descriptors
            desc_void_sym,
            desc_string_sym,
            desc_object_sym,
            desc_class_sym,
            desc_string_array_sym,
            desc_char_array_sym,

            // Class IDs
            java_lang_class_id: OnceCell::new(),
            java_lang_object_id: OnceCell::new(),
        }
    }

    pub fn set_java_lang_class_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.java_lang_class_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("java/lang/Class ID already set".to_string()))
    }

    pub fn set_java_lang_object_id(&self, class_id: ClassId) -> Result<(), JvmError> {
        self.java_lang_object_id
            .set(class_id)
            .map_err(|_| JvmError::Todo("java/lang/Object ID already set".to_string()))
    }

    pub fn get_java_lang_class_id(&self) -> Result<ClassId, JvmError> {
        self.java_lang_class_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("java/lang/Class".to_string()))
    }

    pub fn get_java_lang_object_id(&self) -> Result<ClassId, JvmError> {
        self.java_lang_object_id
            .get()
            .copied()
            .ok_or_else(|| JvmError::Todo("java/lang/Object".to_string()))
    }
}
