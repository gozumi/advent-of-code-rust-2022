use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

use file_system::{get_input_line, DirectoryFacts, get_sum_of_directories_over_100000};

use crate::file_system::{add_size_all_the_way_up, get_size_of_free_space, get_smallest_directory_above};

mod file_system;

fn main() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];

    let lines_buffer = match File::open(file_path) {
        Err(why) => panic!("{}", why),
        Ok(file) => io::BufReader::new(file).lines(),
    };

    process_lines_buffer(lines_buffer);
}

fn process_lines_buffer(lines_buffer: Lines<BufReader<File>>) {
    let mut all_directory_facts: Vec<DirectoryFacts> = vec![];
    let mut current_index: usize = 0;

    for line in lines_buffer {
        let command_line = match line {
            Err(why) => panic!("{}", why),
            Ok(raw_directory_info) => get_input_line(&raw_directory_info),
        };
        
        match command_line {
            file_system::LineOutput::CD(ref directory_name) => {
                if all_directory_facts.len() == 0 {
                    let new_index = all_directory_facts.len();
                    let new_directory_facts = DirectoryFacts {
                        index: new_index,
                        name: String::from(directory_name),
                        size: 0,
                        parent: Option::None,
                    };

                    all_directory_facts.push(new_directory_facts);
                    current_index = new_index;
                } else {
                    if directory_name == ".." {
                        current_index = match all_directory_facts[current_index].parent {
                            Some(parent_index) => parent_index,
                            None => panic!("A parent reference is expected here!")
                        };                      
                    } else {
                        let new_index = all_directory_facts.len();
                        let new_directory_facts = DirectoryFacts {
                            index: new_index,
                            name: String::from(directory_name),
                            size: 0,
                            parent: Option::Some(current_index),
                        };
        
                        all_directory_facts.push(new_directory_facts);
                        current_index = new_index;
                    }
                };

            },
            file_system::LineOutput::LS => {},
            file_system::LineOutput::FileListing { name: _, size } => {
                add_size_all_the_way_up(&mut all_directory_facts, current_index, size);
            },
            file_system::LineOutput::DirectoryListing { name: _ } => {},
        };

        // println!("{:?}", command_line);
    }

    println!("{:#?}", all_directory_facts);

    let total_size_over_100000 = get_sum_of_directories_over_100000(&all_directory_facts);

    println!("Total size over 100000 = {}", total_size_over_100000);

    let free_size = get_size_of_free_space(&all_directory_facts);
    let needed_size = 30000000 - free_size;

    println!("The size of the free space is: {}", free_size);
    println!("The size of the need space is: {}", needed_size);

    let smallest_deletable_directory = get_smallest_directory_above(needed_size, &all_directory_facts);

    println!("The smallest deletable directory is: {:#?}", smallest_deletable_directory);
}
