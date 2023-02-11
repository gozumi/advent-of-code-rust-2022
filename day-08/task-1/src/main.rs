use std::{env::args, fs::File, io::{self, BufRead, Lines, BufReader}};

fn main() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];

    let lines_buffer = match File::open(file_path) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(why) => panic!("Unable to open file {} because {}", file_path, why),
    };

    process_lines_buffer(lines_buffer);
}

fn process_lines_buffer(lines_buffer: Lines<BufReader<File>>) {
    for line in lines_buffer  {
        match line {
            Ok(grid_row) => println!("{}", grid_row),
            Err(why) => panic!("Unable to get grid row: {}", why),
        }
    }
}
