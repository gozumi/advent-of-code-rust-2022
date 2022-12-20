use std::{env, fs::File, io::{self, BufRead}};

use signals::process_datastream;

mod signals;

fn main() {
    let args:Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines_buffer = match File::open(file_path) {
        Err(why) => panic!("{}", why),
        Ok(file) => io::BufReader::new(file).lines()
    };

    for line in lines_buffer  {
        match line {
            Err(why) => panic!("{}", why),
            Ok(datastream) => process_datastream(datastream)
        }

    }
}
