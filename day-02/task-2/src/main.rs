use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Processing file {}", file_path);

    let input = File::open(file_path).unwrap();
    let input_reader = BufReader::new(input);
    let mut my_total_score = 0;

    for round_string in input_reader.lines() {
        let round_string = round_string.unwrap();
        let mut round_split = round_string.split(" ");

        let opponent_choice = round_split.next().unwrap().chars().next().unwrap();
        let win_loose_draw = round_split.next().unwrap().chars().next().unwrap();

        let my_choice = get_my_choice(opponent_choice, win_loose_draw);
        let my_shape_score = get_my_shape_score(my_choice);
        let my_outcome_score = get_my_outcome_score(opponent_choice, my_choice);
        let my_round_score = my_shape_score + my_outcome_score;

        println!(
            "{} {} {}: {} + {} = {}",
            opponent_choice,
            my_choice,
            win_loose_draw,
            my_shape_score,
            my_outcome_score,
            my_round_score
        );

        my_total_score = my_total_score + my_round_score;
    }

    println!("===========================");
    println!("My total score is {}", my_total_score);
}

fn get_my_choice(opponent_choice: char, win_loose_draw: char) -> char {
    return if (opponent_choice == 'B' && win_loose_draw == 'X')
        || (opponent_choice == 'A' && win_loose_draw == 'Y')
        || (opponent_choice == 'C' && win_loose_draw == 'Z')
    {
        'A'
    } else if (opponent_choice == 'C' && win_loose_draw == 'X')
        || (opponent_choice == 'B' && win_loose_draw == 'Y')
        || (opponent_choice == 'A' && win_loose_draw == 'Z')
    {
        'B'
    } else {
        'C'
    };
}

fn get_my_shape_score(my_choice: char) -> i32 {
    return if my_choice == 'A' {
        1
    } else if my_choice == 'B' {
        2
    } else if my_choice == 'C' {
        3
    } else {
        0
    };
}

fn get_my_outcome_score(opponent_choice: char, my_choice: char) -> i32 {
    return if (opponent_choice == 'A' && my_choice == 'B')
        || (opponent_choice == 'B' && my_choice == 'C')
        || (opponent_choice == 'C' && my_choice == 'A')
    {
        6
    } else if (opponent_choice == 'A' && my_choice == 'A')
        || (opponent_choice == 'B' && my_choice == 'B')
        || (opponent_choice == 'C' && my_choice == 'C')
    {
        3
    } else {
        0
    };
}
