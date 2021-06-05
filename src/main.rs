mod riscv;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(" Usage: rv64_emulator <filename>");
    }
    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut encoded_instructions = Vec::new();
    match file.read_to_end(&mut encoded_instructions) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(bytes_read) => bytes_read,
    };
    riscv::emulate(encoded_instructions);
}
