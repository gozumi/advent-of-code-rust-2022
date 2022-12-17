use std::{env, fs::File, io::{self, BufRead}};

use stacks::process_input;


mod  stacks;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines_buffer = match File::open(file_path) {
        Err(why) => panic!("{}", why),
        Ok(file) => io::BufReader::new(file).lines()
    };

    process_input(lines_buffer);
}
