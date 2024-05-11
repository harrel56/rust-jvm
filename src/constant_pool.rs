use crate::parser::ByteParser;

#[derive(Debug)]
pub enum ConstantInfo {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
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
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16
    },
    Module {
        name_index: u16
    },
    Package {
        name_index: u16
    }
}

pub fn read_cp_info(parser: &mut ByteParser) -> ConstantInfo {
    let tag = parser.read_u8();
    match tag {
        1 => {
            let length = parser.read_u16();
            let mut bytes = Vec::with_capacity(length as usize);
            for _ in 0..length {
                bytes.push(parser.read_u8());
            }
            ConstantInfo::Utf8(String::from_utf8(bytes).unwrap())
        }
        3 => {
            ConstantInfo::Integer(parser.read_u32() as i32)
        }
        4 => {
            ConstantInfo::Float(parser.read_u32() as f32)
        }
        5 => {
            ConstantInfo::Long(parser.read_u64() as i64)
        }
        6 => {
            ConstantInfo::Double(parser.read_u64() as f64)
        }
        7 => {
            let name_index = parser.read_u16();
            ConstantInfo::Class { name_index }
        }
        8 => {
            let string_index = parser.read_u16();
            ConstantInfo::String { string_index }
        }
        9 => {
            let class_index = parser.read_u16();
            let name_and_type_index = parser.read_u16();
            ConstantInfo::Fieldref { class_index, name_and_type_index }
        }
        10 => {
            let class_index = parser.read_u16();
            let name_and_type_index = parser.read_u16();
            ConstantInfo::Methodref { class_index, name_and_type_index }
        }
        11 => {
            let class_index = parser.read_u16();
            let name_and_type_index = parser.read_u16();
            ConstantInfo::InterfaceMethodref { class_index, name_and_type_index }
        }
        12 => {
            let name_index = parser.read_u16();
            let descriptor_index = parser.read_u16();
            ConstantInfo::NameAndType { name_index, descriptor_index }
        }
        15 => {
            let reference_kind = parser.read_u8();
            let reference_index = parser.read_u16();
            ConstantInfo::MethodHandle { reference_kind, reference_index }
        }
        16 => {
            let descriptor_index = parser.read_u16();
            ConstantInfo::MethodType { descriptor_index }
        },
        17 => {
            let bootstrap_method_attr_index = parser.read_u16();
            let name_and_type_index = parser.read_u16();
            ConstantInfo::Dynamic { bootstrap_method_attr_index, name_and_type_index }
        }
        18 => {
            let bootstrap_method_attr_index = parser.read_u16();
            let name_and_type_index = parser.read_u16();
            ConstantInfo::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index }
        },
        19 => {
            let name_index = parser.read_u16();
            ConstantInfo::Module { name_index }
        },
        20 => {
            let name_index = parser.read_u16();
            ConstantInfo::Package { name_index }
        }
        _ => panic!("Invalid cp_info.tag [{}]", tag)
    }
}