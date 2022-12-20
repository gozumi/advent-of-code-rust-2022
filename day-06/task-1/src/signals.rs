pub fn process_datastream(datastream: String) {
    println!("{} length {}", datastream, datastream.len());
    println!("=================================");

    let characters: Vec<char> = datastream.chars().collect();
    let mut four_most_recent_chars = String::from(&datastream[0..4]);

    for i in 4..characters.len() {
        println!(">>>>> {} {}", four_most_recent_chars, i);

        if is_unique_sequence(&four_most_recent_chars) {
            println!("The first start of the first packet is at {}", i);
            break;
        }

        four_most_recent_chars = String::from(&four_most_recent_chars[1..]);
        four_most_recent_chars.push(characters[i]);
    }
}

fn is_unique_sequence(sequence: &str) -> bool {
    let sequence_vector: Vec<char> = sequence.chars().collect();
    let mut items_removed = 0;
    
    for i in 0..sequence.len() {
        let sequence_string = String::from(sequence);

        let shortened_sequence = sequence_string.replace(sequence_vector[i],  "");

        items_removed += sequence.len() - shortened_sequence.len();
        
    }

    return items_removed == sequence.len();
}