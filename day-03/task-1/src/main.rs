use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("The input file is {}.", file_path);

    if let Ok(lines) = read_lines(file_path) {
        let mut sum_of_common_item_priorities: i32 = 0;
        let mut sum_of_group_badges_priorities: i32 = 0;
        let mut group = -1;
        let mut elf_id_in_group = 0;
        let mut group_rucksacks = (String::from(""),String::from(""), String::from(""));

        for line in lines {
            if let Ok(rucksack) = line {
                let rucksack_size = rucksack.len();
                let compartment_size = rucksack_size / 2;
                let compartments = rucksack.split_at(compartment_size);
                let common_item = find_common_item(compartments.0, compartments.1);
                let common_item_priority = get_item_priority(common_item);
                sum_of_common_item_priorities += common_item_priority;

                println!(
                    "elf {}: {} {}, {}, {}",
                    elf_id_in_group,
                    rucksack_size,
                    compartment_size,
                    common_item,
                    common_item_priority
                );
                println!("{:#?}", compartments);
                println!("{}", (&rucksack[..]).clone());

                match elf_id_in_group {
                    0 => group_rucksacks.0 = rucksack.clone(),
                    1 => group_rucksacks.1 = rucksack.clone(),
                    2 => group_rucksacks.2 = rucksack.clone(),
                    _ => panic!("Elf should have number 1, 2 or 3")
                }

                if ((elf_id_in_group + 1) % 3) == 0 {
                    elf_id_in_group = 0;
                    group += 1;
                    
                    
                    println!("=======================================================");
                    println!("Group {}", group);
                    println!("elf 0 rucksack {}", group_rucksacks.0);
                    println!("elf 1 rucksack {}", group_rucksacks.1);
                    println!("elf 2 rucksack {}", group_rucksacks.2);
                    
                    let group_badge = get_group_badge(group_rucksacks.clone());
                    let group_badge_priority = get_item_priority(group_badge);
                    sum_of_group_badges_priorities += group_badge_priority;

                    println!("The group badge is {} {}", group_badge, get_item_priority(group_badge));
                    

                    println!("=======================================================");
                    println!("");
                } else {
                    elf_id_in_group += 1;
                }
            }
        }

        println!("");
        println!("===================================================================");
        println!(
            "Sum of common item priorities {}",
            sum_of_common_item_priorities
        );
        println!(
            "Sum of group badges priorities {}",
            sum_of_group_badges_priorities
        );
        println!("===================================================================");
    }
}

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_common_item(compartment_1: &str, compartment_2: &str) -> char {
    let compartment_1_items = compartment_1.chars();
    let compartment_2_items = compartment_2.chars();

    for item_in_1 in compartment_1_items {
        for item_in_2 in compartment_2_items.clone() {
            if item_in_1 == item_in_2 {
                return item_in_1;
            }
        }
    }

    return ' ';
}

fn get_item_priority(item: char) -> i32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return (alphabet.chars().position(|c| c == item).unwrap() as i32) + 1;
}

fn get_group_badge(group_rucksacks: (String, String, String)) -> char {
    let elf_0_rucksack_items = group_rucksacks.0.chars();
    let elf_1_rucksack_items = group_rucksacks.1.chars();
    let elf_2_rucksack_items = group_rucksacks.2.chars();

    for elf_0_item in elf_0_rucksack_items {
        for elf_1_item in elf_1_rucksack_items.clone() {
            if elf_0_item == elf_1_item {
                for elf_2_item in elf_2_rucksack_items.clone() {
                    if elf_1_item == elf_2_item {
                        return elf_2_item;
                    }
                }
            }
        }
    }

    panic!("Unable to find badge for group");
}