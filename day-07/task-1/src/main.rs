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
    let mut root_directory_tree = Node::DirectoryNode {
        directoryName: String::from("/"),
        size: 0,
        children: vec![],
        parent: &Node::None,
    };

    let mut current_directory_node = &mut root_directory_tree;

    for line in lines_buffer {
        let command_line = match line {
            Err(why) => panic!("{}", why),
            Ok(raw_directory_info) => get_input_line(&raw_directory_info),
        };

        match command_line {
            LineOutput::CD(directory_name) => {
                if directory_name == "/" {
                    // current_directory_node = &mut root_directory_tree;
                    continue;
                } else {
                    let new_directory_node = Node::DirectoryNode { directoryName: directory_name.clone(), size: 0, children: vec![], parent: &mut current_directory_node };
                    match current_directory_node {
                        Node::DirectoryNode { directoryName, size, children, parent } => {
                            children.push(new_directory_node)
                        },
                        _ => { panic!("{} should be a Node::DirectoryNode", directory_name ) }
                    }
                }
                // println!("change directory to {}", directory_name)
            },
            LineOutput::LS => {
                print!("")
            },
            _ => print!("")
            // LineOutput::DirectoryListing { name } => match current_directory_node {
            //     Node::DirectoryNode {
            //         name,
            //         size,
            //         children,
            //     } => {
            //         let new_node = Node::DirectoryNode {
            //             name,
            //             size: 0,
            //             children: vec![],
            //         };
            //         children.push(new_node);
            //     }
            //     Node::FileNode { name, size } => todo!(),
            // },
            // LineOutput::LS => todo!(),
            // LineOutput::FileListing { name, size } => todo!(),
        }

        // println!("{:?}", command_line);
    }

    println!("=================================");
    println!("Directory tree");
    println!("=================================");

    print_directory_tree(root_directory_tree);
}
