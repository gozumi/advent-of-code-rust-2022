use std::{
    fs::File,
    io::{BufReader, Lines},
};

struct Move {
    n: i8,
    from: usize,
    to: usize,
}

pub fn process_input(lines_buffer: Lines<BufReader<File>>) {
    let mut stacks: [Vec<char>; 10] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let mut is_stack_data_complete = false;
    
    for line in lines_buffer {
        let slice = match line {
            Err(why) => panic!("{:?}", why),
            Ok(slice) => slice,
        };

        let compressed_slice = slice.replace(" ", "");
        if compressed_slice == "123456789" {
            stacks = reverse_stacks(stacks);
            process_end_of_stacks(&stacks);
            is_stack_data_complete = true;
        } else if !is_stack_data_complete {
            stacks = add_slice_to_stacks(&slice, stacks);
        } else if compressed_slice == "" {
            println!("----------");
        } else {
            let the_move = get_move(&slice);
            println!("=========================");
            println!("slice -> {}", slice);
            perform_move(the_move, &mut stacks);
            print_stacks(&stacks);
        }
    }
    
    let tops_of_stacks = get_tops_of_stacks(&mut stacks);
    println!("");
    println!("===========================");
    println!("Tops of stacks => {}", tops_of_stacks);
    println!("===========================");
}

fn process_end_of_stacks(stacks: &[Vec<char>; 10]) {
    println!("=========================");
    println!("End of stacks input");
    println!("-------------------------");
    print_stacks(stacks);
}

fn add_slice_to_stacks(slice: &String, mut stacks: [Vec<char>; 10]) -> [Vec<char>; 10] {
    let slice_vetor: Vec<char> = slice.chars().collect();

    add_crate_to_stack(slice_vetor[1], &mut stacks[1]);
    add_crate_to_stack(slice_vetor[5], &mut stacks[2]);
    add_crate_to_stack(slice_vetor[9], &mut stacks[3]);
    add_crate_to_stack(slice_vetor[13], &mut stacks[4]);
    add_crate_to_stack(slice_vetor[17], &mut stacks[5]);
    add_crate_to_stack(slice_vetor[21], &mut stacks[6]);
    add_crate_to_stack(slice_vetor[25], &mut stacks[7]);
    add_crate_to_stack(slice_vetor[29], &mut stacks[8]);
    add_crate_to_stack(slice_vetor[33], &mut stacks[9]);

    println!(
        "{} {} {} {} {} {} {} {} {}",
        slice_vetor[1],
        slice_vetor[5],
        slice_vetor[9],
        slice_vetor[13],
        slice_vetor[17],
        slice_vetor[21],
        slice_vetor[25],
        slice_vetor[29],
        slice_vetor[33],
    );

    return stacks;
}

fn add_crate_to_stack(the_crate: char, the_stack: &mut Vec<char>) {
    if the_crate != ' ' {
        the_stack.push(the_crate);
    }
}

fn reverse_stacks(mut stacks: [Vec<char>; 10]) -> [Vec<char>; 10] {
    stacks[1].reverse();
    stacks[2].reverse();
    stacks[3].reverse();
    stacks[4].reverse();
    stacks[5].reverse();
    stacks[6].reverse();
    stacks[7].reverse();
    stacks[8].reverse();
    stacks[9].reverse();

    return stacks;
}

fn get_move(slice: &String) -> Move {
    let binding = slice
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "");
    let mut stack_move_iterator = binding.trim().split(' ');

    return Move {
        n: stack_move_iterator.next().unwrap().parse::<i8>().unwrap(),
        from: stack_move_iterator
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        to: stack_move_iterator
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    };
}

fn perform_move(the_move: Move, stacks: &mut [Vec<char>; 10]) {
    println!("-----------");
    println!("({}, {}, {})", the_move.n, the_move.from, the_move.to);
    println!("-----------");

    for _i in 1..(the_move.n+1) {
        let crate_to_be_moved = stacks[the_move.from].pop().unwrap();
        stacks[the_move.to].push(crate_to_be_moved);
    }
}

fn print_stacks(stacks: &[Vec<char>; 10]) {
    println!("1) {:?}", stacks[1]);
    println!("2) {:?}", stacks[2]);
    println!("3) {:?}", stacks[3]);
    println!("4) {:?}", stacks[4]);
    println!("5) {:?}", stacks[5]);
    println!("6) {:?}", stacks[6]);
    println!("7) {:?}", stacks[7]);
    println!("8) {:?}", stacks[8]);
    println!("9) {:?}", stacks[9]);
}

fn get_tops_of_stacks(stacks: &mut[Vec<char>; 10]) -> String {
    let mut result = String::from("");

    for i in 1..10  {
        result.push(stacks[i].pop().unwrap());
    }
    
    return result;
}