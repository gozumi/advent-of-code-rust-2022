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

    let mut elf_id = 0;
    let mut calorie_count = 0;
    let mut elf_carrying_most_calories = 0;
    let mut most_calories_carried = 0;

    for meal in input_reader.lines() {
        let meal = meal.unwrap();

        if meal == "" {
            elf_id = elf_id + 1;
            
            println!("elf {} calories {}", elf_id, calorie_count);
            
            if calorie_count > most_calories_carried {
                most_calories_carried = calorie_count;
                elf_carrying_most_calories = elf_id;
            }

            calorie_count = 0;
        } else {
            calorie_count = calorie_count + meal.parse::<i32>().unwrap();
        }
    }

    println!("================================");
    println!(
        "The elf carrying the most calories is {} carring {} calories!",
        elf_carrying_most_calories, most_calories_carried
    );
}
