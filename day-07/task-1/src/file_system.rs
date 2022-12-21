#[derive(Debug)]
pub enum LineOutput {
    CD(String),
    LS,
    FileListing {
            name: String,
            size: i32
    },
    DirectoryListing {
        name: String
    }
}

#[derive(Debug)]
pub struct DirectoryFacts {
    pub index: usize,
    pub name: String,
    pub size: i32,
    pub parent: Option<usize>
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
        return LineOutput::DirectoryListing { name: input_parts[1].clone() }
    } else {
        return LineOutput::FileListing { name: input_parts[1].clone(), size: input_parts[0].parse::<i32>().unwrap()};
    }
}

pub fn add_size_all_the_way_up(all_directory_facts: &mut Vec<DirectoryFacts>, current_index: usize, size: i32) {
    all_directory_facts[current_index].size += size;
    match all_directory_facts[current_index].parent {
        Some(parent_index) => add_size_all_the_way_up(all_directory_facts, parent_index, size),
        None => {},
    }
}

pub fn get_sum_of_directories_over_100000(all_directory_facts: &Vec<DirectoryFacts>) -> i32 {
    let mut total = 0;

    for directory_facts in all_directory_facts {
        if directory_facts.size <= 100000 {
            total += directory_facts.size;
        }
    }

    return total;
}

pub fn get_size_of_free_space(all_directory_facts: &Vec<DirectoryFacts>) -> i32 {
    let used_size  = all_directory_facts[0].size;

    return 70000000 - used_size;
}

pub fn get_smallest_directory_above(needed_size: i32, all_directory_facts: &Vec<DirectoryFacts>) -> &DirectoryFacts {
    let mut smallest_directory_above_index: usize = 0;

    for directory_facts in all_directory_facts  {
        if (directory_facts.size >= needed_size) && (directory_facts.size < all_directory_facts[smallest_directory_above_index].size) {
            smallest_directory_above_index = directory_facts.index;
        } 
    }

    return &all_directory_facts[smallest_directory_above_index];
}
