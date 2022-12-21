use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

use file_system::{get_input_line, print_directory_tree, Node};

use crate::file_system::LineOutput;

mod file_system;

fn main() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];

    let lines_buffer = match File::open(file_path) {
        Err(why) => panic!("{}", why),
        Ok(file) => io::BufReader::new(file).lines(),
    };

    build_directory_tree(lines_buffer);
}

fn build_directory_tree(lines_buffer: Lines<BufReader<File>>) {
    let root_directory_tree = Node::DirectoryNode {
        directoryName: String::from("/"),
        size: 0,
        children: vec![],
    };

    // let mut current_directory_node = &root_directory_tree;

    for line in lines_buffer {
        let command_line = match line {
            Err(why) => panic!("{}", why),
            Ok(raw_directory_info) => get_input_line(&raw_directory_info),
        };

    //     match command_line {
    //         LineOutput::CD(name) => {
    //             if name == "/" {
    //                 current_directory_node = &root_directory_tree;
    //             }
    //         }
    //         LineOutput::DirectoryListing { name } => match current_directory_node {
    //             Node::DirectoryNode {
    //                 name,
    //                 size,
    //                 children,
    //             } => {
    //                 let new_node = Node::DirectoryNode {
    //                     name,
    //                     size: 0,
    //                     children: vec![],
    //                 };
    //                 children.push(new_node);
    //             }
    //             Node::FileNode { name, size } => todo!(),
    //         },
    //         LineOutput::LS => todo!(),
    //         LineOutput::FileListing { name, size } => todo!(),
    //     }

        println!("{:?}", command_line);
    }

    print_directory_tree(root_directory_tree);
}
