use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

use elf_assignment::process_lines;

mod elf_assignment;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines_buffer = match File::open(file_path) {
        Err(why) => panic!("{:?}", why),
        Ok(file) => io::BufReader::new(file).lines(),
    };

    process_lines(lines_buffer)
}
