use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn process_lines(lines_buffer: Lines<BufReader<File>>) {
    let mut number_of_contained_sections = 0;
    for line in lines_buffer {
        let elf_pair_sections = match line {
            Err(why) => panic!("{:?}", why),
            Ok(assigned_sections) => process_assigned_sections(assigned_sections),
        };

        println!("{:?}", elf_pair_sections);

        if do_sections_0_contain_sections_1(elf_pair_sections.0, elf_pair_sections.1)
            || (do_sections_0_contain_sections_1(elf_pair_sections.1, elf_pair_sections.0))
        {
            number_of_contained_sections += 1;
        }
    }

    println!("================================================");
    println!(
        "Number of full contained section ranges: {}",
        number_of_contained_sections
    );
}

fn process_assigned_sections(assigned_sections: String) -> ((i16, i16), (i16, i16)) {
    let mut assigned_sections_iterator = assigned_sections.split(",");

    return (
        get_assigned_sections(assigned_sections_iterator.next().unwrap()),
        get_assigned_sections(assigned_sections_iterator.next().unwrap()),
    );
}

fn get_assigned_sections(single_elf_assigned_sections: &str) -> (i16, i16) {
    let mut single_elf_assigned_sections_iterator = single_elf_assigned_sections.split("-");

    return (
        single_elf_assigned_sections_iterator
            .next()
            .unwrap()
            .parse::<i16>()
            .unwrap(),
        single_elf_assigned_sections_iterator
            .next()
            .unwrap()
            .parse::<i16>()
            .unwrap(),
    );
}

fn do_sections_0_contain_sections_1(sections_0: (i16, i16), sections_1: (i16, i16)) -> bool {
    return (sections_0.0 <= sections_1.0 && sections_0.1 >= sections_1.1)
        || (sections_0.0 >= sections_1.0 && sections_0.1 <= sections_1.1);
}
