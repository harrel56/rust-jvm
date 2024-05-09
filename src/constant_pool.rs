enum ConstantInfo {
    Utf8 {
        bytes: [u8]
    },
    Integer {
        bytes: u32
    },
    Float {
        bytes: u32
    },
    Long {
        high_bytes: u32,
        low_bytes: u32
    },
    Double {
        high_bytes: u32,
        low_bytes: u32
    },
    Class {
        name_index: u16
    },
    String {
        string_index: u16
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16
    },
    MethodType {
        descriptor_index: u16
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        
    },
}

fn x() {
    let x : ConstantInfo;
    match x {
        ConstantInfo::Utf8 => {}
        ConstantInfo::Integer(_) => {}
        ConstantInfo::Float(_) => {}
        ConstantInfo::Long(_) => {}
        ConstantInfo::Double(_) => {}
        ConstantInfo::Class(_) => {}
        ConstantInfo::String(_) => {}
        ConstantInfo::Fieldref(_) => {}
        ConstantInfo::Methodref(_) => {}
        ConstantInfo::InterfaceMethodref(_) => {}
        ConstantInfo::NameAndType(_) => {}
        ConstantInfo::MethodHandle(_) => {}
        ConstantInfo::MethodType(_) => {}
        ConstantInfo::InvokeDynamic(_) => {}
    }
}