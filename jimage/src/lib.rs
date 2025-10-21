use common::utils::cursor::{ByteCursor, ByteOrder};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

const HASH_MUL: u32 = 0x01_00_01_93;

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

#[derive(Debug)]
struct Entry {
    module_off: u32,
    parent_off: u32,
    base_off: u32,
    ext_off: u32,
    content_off: u64,
    compressed_size: u64,
    uncompressed_size: u64,
}

pub struct JImage {
    mmap: Mmap,
    pub header: Header,
    redirect_off: usize,
    offsets_off: usize,
    locations_off: usize,
    strings_off: usize,
    data_base: usize,
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

        let header_size = cur.position();
        let redirect_off = header_size;
        let offsets_off = redirect_off + (header.table_length as usize) * 4;
        let locations_off = offsets_off + (header.table_length as usize) * 4;
        let strings_off = locations_off + (header.locations_size as usize);
        let data_base = strings_off + (header.strings_size as usize);

        Self {
            mmap,
            header,
            redirect_off,
            offsets_off,
            locations_off,
            strings_off,
            data_base,
        }
    }

    fn hash_seeded(seed: u32, name: &str) -> u32 {
        let mut h = seed;
        for &b in name.as_bytes() {
            h = h.wrapping_mul(HASH_MUL) ^ (b as u32);
        }
        h & 0x7FFF_FFFF
    }
    fn hash(&self, name: &str) -> u32 {
        Self::hash_seeded(HASH_MUL, name)
    }

    fn redirect_at(&self, i: u32) -> i32 {
        let o = self.redirect_off + (i as usize) * 4;
        let b = self.mmap[o..o + 4].try_into().unwrap();
        i32::from_le_bytes(b)
    }
    fn offset_at(&self, i: u32) -> u32 {
        let o = self.offsets_off + (i as usize) * 4;
        let b = self.mmap[o..o + 4].try_into().unwrap();
        u32::from_le_bytes(b)
    }

    fn lookup_slot(&self, path: &str) -> Option<(u32, usize)> {
        let len = self.header.table_length;
        if len == 0 {
            return None;
        }
        let idx = self.hash(path) % len;
        let r = self.redirect_at(idx);
        let slot = if r == 0 {
            return None;
        } else if r < 0 {
            (-1 - r) as u32
        } else {
            Self::hash_seeded(r as u32, path) % len
        };
        let loc_off = self.offset_at(slot) as usize;
        if loc_off == 0 || loc_off >= self.header.locations_size as usize {
            return None;
        }
        Some((slot, loc_off))
    }

    fn decode_location(&self, loc_off: usize) -> Entry {
        let loc = &self.mmap[self.locations_off + loc_off
            ..self.locations_off + self.header.locations_size as usize];
        let mut c = ByteCursor::with_order(loc, ByteOrder::BigEndian);

        let mut v = [0u64; 8];
        loop {
            let tag = c.u8().unwrap();
            if tag <= 0x07 {
                break;
            }
            let kind = (tag >> 3) as usize;
            let len = (tag & 0x07) as usize + 1;
            let s = c.slice(len).unwrap();
            let mut x = 0u64;
            for &b in s {
                x = (x << 8) | (b as u64);
            }
            if kind < v.len() {
                v[kind] = x;
            }
        }

        Entry {
            module_off: v[1] as u32,
            parent_off: v[2] as u32,
            base_off: v[3] as u32,
            ext_off: v[4] as u32,
            content_off: v[5],
            compressed_size: v[6],
            uncompressed_size: v[7],
        }
    }

    fn string_at(&self, off: usize) -> String {
        let s = &self.mmap
            [self.strings_off + off..self.strings_off + self.header.strings_size as usize];
        let end = s.iter().position(|&b| b == 0).unwrap();
        String::from_utf8_lossy(&s[..end]).into_owned()
    }

    fn make_name(&self, e: &Entry) -> String {
        let mut name = String::new();
        if e.module_off != 0 {
            name.push('/');
            name.push_str(&self.string_at(e.module_off as usize));
            name.push('/');
        }
        if e.parent_off != 0 {
            name.push_str(&self.string_at(e.parent_off as usize));
            name.push('/');
        }
        name.push_str(&self.string_at(e.base_off as usize));
        if e.ext_off != 0 {
            name.push('.');
            name.push_str(&self.string_at(e.ext_off as usize));
        }
        name
    }

    pub fn open_java_base_class(&self, path: &str) -> Option<&[u8]> {
        let full_name = format!("/java.base/{path}.class");
        let (_, loc_off) = self.lookup_slot(&full_name)?;
        let e = self.decode_location(loc_off);
        // verify name, just in case
        if self.make_name(&e) != full_name {
            return None;
        }

        let start = self.data_base + (e.content_off as usize);
        if e.compressed_size != 0 {
            unimplemented!("compressed resources not supported yet");
        }
        let end = start + (e.uncompressed_size as usize);
        Some(&self.mmap[start..end])
    }
}
