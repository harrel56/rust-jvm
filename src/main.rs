mod parser;
mod constant_pool;
mod attribute;

use std::{fs};
use std::io::{Error, ErrorKind};
use crate::attribute::{Attribute, read_attributes};
use crate::constant_pool::{ConstantInfo, ConstantPool, read_cp_info};
use crate::parser::ByteParser;

#[derive(Debug)]
struct ClassHeader {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<ConstantInfo>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<MemberInfo>,
    methods: Vec<MemberInfo>,
    attributes: Vec<Attribute>
}

#[derive(Debug)]
struct MemberInfo {
    access_flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<Attribute>
}

fn main() -> Result<(), Error> {
    let bytes = fs::read("./src/Main.class")?;

    let mut parser = ByteParser::new(&bytes);
    let magic = parser.read_u32();

    if magic != 0xcafebabe {
        return Err(Error::new(ErrorKind::InvalidData, "magic bytes are invalid"));
    }

    let minor_version = parser.read_u16();
    let major_version = parser.read_u16();

    let constant_pool_size = parser.read_u16() - 1;
    let mut constant_pool = Vec::with_capacity(constant_pool_size as usize);
    for _ in 0..constant_pool_size {
        constant_pool.push(read_cp_info(&mut parser));
    }

    let access_flags = parser.read_u16();
    let this_class = parser.read_u16();
    let super_class = parser.read_u16();

    let interfaces_count = parser.read_u16();
    let mut interfaces = Vec::with_capacity(interfaces_count as usize);
    for _ in 0..interfaces_count {
        interfaces.push(parser.read_u16());
    }

    let fields = read_member_infos(&mut parser, &constant_pool);
    let methods = read_member_infos(&mut parser, &constant_pool);
    let attributes = read_attributes(&mut parser);

    let class_header = ClassHeader {
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes
    };
    println!("class: {:?}", class_header);
    Ok(())
}

fn read_member_infos(parser: &mut ByteParser, cp: &Vec<ConstantInfo>) -> Vec<MemberInfo> {
    let members_count = parser.read_u16();
    let mut members = Vec::with_capacity(members_count as usize);
    for _ in 0..members_count {
        let access_flags = parser.read_u16();
        let name_index = parser.read_u16();
        let descriptor_index = parser.read_u16();
        let attributes = read_attributes(parser);

        let name = cp.as_string(name_index);
        let descriptor = cp.as_string(descriptor_index);
        members.push(MemberInfo {
            access_flags,
            name,
            descriptor,
            attributes,
        });
    }
    members
}

fn cp_as_string(info: &ConstantInfo) -> String {
    match info {
        ConstantInfo::Utf8(data) => data.clone(),
        _ => panic!()
    }
}



