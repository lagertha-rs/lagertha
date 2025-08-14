use class_file::ClassFile;
use rt_model::class::Class;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("Main.class").expect("Can't open Main.class");
    //let mut file = File::open("Main.class").expect("Can't open Main.class");
    let m = file.metadata().expect("Metadata err");
    let mut buf = Vec::with_capacity(m.len() as usize);
    file.read_to_end(&mut buf).expect("Problem with read");

    let class = ClassFile::try_from(buf).unwrap();
    println!("{}", class);
    println!("---------------------");
    let class = Class::new(class).unwrap();
    println!("{class:?}")
}
