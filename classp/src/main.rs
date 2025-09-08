use class_file::ClassFile;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("System.class").expect("Can't open System.class");
    let m = file.metadata().expect("Metadata err");
    let mut buf = Vec::with_capacity(m.len() as usize);
    file.read_to_end(&mut buf).expect("Problem with read");

    let class = ClassFile::try_from(buf).unwrap();

    println!("{class}");
}
