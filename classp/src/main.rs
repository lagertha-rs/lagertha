use class_file::ClassFile;
use std::fs::File;
use std::io::Read;

fn handle_class_file_arg() -> String {
    std::env::args()
        .nth(1)
        .expect("Please provide a class file as argument")
}

fn main() {
    let class_file_path = handle_class_file_arg();
    let mut file =
        File::open(&class_file_path).unwrap_or_else(|_| panic!("Cannot open {class_file_path}"));
    let m = file.metadata().expect("Metadata err");
    let mut buf = Vec::with_capacity(m.len() as usize);
    file.read_to_end(&mut buf).expect("Problem with read");

    let class = ClassFile::try_from(buf).unwrap();

    print!("{class}");
}
