pub enum Node {
    DirectoryNode {
        directoryName: String,
        size: i32,
        children: Vec<Node>
    },
    FileNode {
        fileName: String,
        size: i32,
    }
}

#[derive(Debug)]
pub enum LineOutput {
    CD(String),
    LS,
    FileListing {
            fileName: String,
            size: i32
    },
    DirectoryListing {
        directoryName: String
    }
}

pub fn get_input_line(raw_input: &str) -> LineOutput {
    let raw_parts = raw_input.split(" ");

    let mut input_parts:Vec<String> = vec![];

    for part in raw_parts  {
        input_parts.push(String::from(part));
    }

    if input_parts[0] == "$" && input_parts[1] == "cd" {
        return LineOutput::CD(input_parts[2].clone());
    } else if input_parts[0] == "$" && input_parts[1] == "ls" {
        return LineOutput::LS;
    } else if input_parts[0] == "dir" {
        return LineOutput::DirectoryListing { directoryName: input_parts[1].clone() }
    } else {
        return LineOutput::FileListing { fileName: input_parts[1].clone(), size: input_parts[0].parse::<i32>().unwrap()};
    }
}

pub fn print_directory_tree(node: Node) {
    println!("=================================");
    println!("Directory tree");
    println!("=================================");
    match node {
        Node::DirectoryNode { directoryName, size, children } => {
            println!("{}", directoryName);
            for n in children  {
                print_directory_tree(n)
            }
        },
        Node::FileNode { fileName, size } => println!("{} {}", fileName, size)
    }
}