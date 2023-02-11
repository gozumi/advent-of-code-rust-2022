use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

use forest_grid::{get_grid_row, VisibilityCount};

use crate::forest_grid::get_visibility_count;

mod forest_grid;

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
    let mut grid: Vec<Vec<u32>> = vec![];
    let mut visibility_count = VisibilityCount::None;

    for line in lines_buffer {
        let grid_row = match line {
            Ok(grid_row_string) => get_grid_row(grid_row_string.as_str()),
            Err(why) => panic!("Unable to get grid row: {}", why),
        };

        visibility_count = get_visibility_count(&grid_row, visibility_count);

        // println!("{:?}", grid_row);
        grid.push(grid_row);
    }

    println!("{:#?}", visibility_count);
}
