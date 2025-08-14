use class::create_runtime;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("Main.class").expect("Can't open Main.class");
    let m = file.metadata().expect("Metadata err");
    let mut buf = Vec::with_capacity(m.len() as usize);
    file.read_to_end(&mut buf).expect("Problem with read");

    let rt = create_runtime(buf).unwrap();
    println!("{rt}");
}
