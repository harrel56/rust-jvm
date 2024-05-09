#[derive(Debug)]
enum ConstantInfo {
    Utf8 {
        bytes: Vec<u8>
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
        name_and_type_index: u16
    },
}

impl ConstantInfo {
    pub fn tag(&self) -> u8 {
        match self {
            ConstantInfo::Utf8 => 1,
            ConstantInfo::Integer => 3,
            ConstantInfo::Float => 4,
            ConstantInfo::Long => 5,
            ConstantInfo::Double => 6,
            ConstantInfo::Class => 7,
            ConstantInfo::String => 8,
            ConstantInfo::Fieldref => 9,
            ConstantInfo::Methodref => 10,
            ConstantInfo::InterfaceMethodref => 11,
            ConstantInfo::NameAndType => 12,
            ConstantInfo::MethodHandle => 15,
            ConstantInfo::MethodType => 16,
            ConstantInfo::InvokeDynamic => 18
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ConstantInfo::Utf8 => "CONSTANT_Utf8",
            ConstantInfo::Integer => "CONSTANT_Integer",
            ConstantInfo::Float => "CONSTANT_Float",
            ConstantInfo::Long => "CONSTANT_Long",
            ConstantInfo::Double => "CONSTANT_Double",
            ConstantInfo::Class => "CONSTANT_Class",
            ConstantInfo::String => "CONSTANT_String",
            ConstantInfo::Fieldref => "CONSTANT_Fieldref",
            ConstantInfo::Methodref => "CONSTANT_Methodref",
            ConstantInfo::InterfaceMethodref => "CONSTANT_InterfaceMethodref",
            ConstantInfo::NameAndType => "CONSTANT_NameAndType",
            ConstantInfo::MethodHandle => "CONSTANT_MethodHandle",
            ConstantInfo::MethodType => "CONSTANT_MethodType",
            ConstantInfo::InvokeDynamic => "CONSTANT_InvokeDynamic"
        }
    }
}