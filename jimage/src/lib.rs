use common::utils::cursor::{ByteCursor, ByteOrder};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct Header {
    pub magic: u32,
    pub major: u16,
    pub minor: u16,
    pub flags: u32,
    pub resource_count: u32,
    pub table_length: u32,
    pub locations_size: u32,
    pub strings_size: u32,
}

pub struct JImage {
    mmap: Mmap,
    pub header: Header,
}

impl JImage {
    pub fn new<P: AsRef<Path>>(p: P) -> Self {
        let file = File::open(p).unwrap();
        let mmap = unsafe { Mmap::map(&file).unwrap() };
        let mut cur = ByteCursor::with_order(&mmap, ByteOrder::LittleEndian);

        let magic = cur.u32().unwrap();
        assert_eq!(magic, 0xCAFEDADA, "bad magic: 0x{magic:08X}");

        let ver = cur.u32().unwrap();
        let major = (ver >> 16) as u16;
        let minor = (ver & 0xFFFF) as u16;

        let header = Header {
            magic,
            major,
            minor,
            flags: cur.u32().unwrap(),
            resource_count: cur.u32().unwrap(),
            table_length: cur.u32().unwrap(),
            locations_size: cur.u32().unwrap(),
            strings_size: cur.u32().unwrap(),
        };

        println!("JImage Header: {:?}", header);

        Self { mmap, header }
    }
}
