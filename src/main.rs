use std::io::{
    Read,
};

fn main() {
    let mut args = std::env::args();
    let source_filename = args.nth(1).expect("requires source file name");
    let mut source_file = std::fs::File::open(source_filename).unwrap();
    let mut source_buf = String::new();
    source_file.read_to_string(&mut source_buf).unwrap();

    let source = source_buf;

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    for char in source.chars() {
        match char {
            '+' => println!("plus"),
            _ => {}
        }
    }

}
