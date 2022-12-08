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
    let mut top3 = [(0, 0), (0, 0), (0, 0)];
    let mut calorie_count = 0;

    for meal in input_reader.lines() {
        let meal = meal.unwrap();

        if meal == "" {
            elf_id = elf_id + 1;

            println!("elf {} calories {}", elf_id, calorie_count);

            if calorie_count > top3[0].1 {
                top3[2] = top3[1];
                top3[1] = top3[0];
                
                top3[0] = assign_elf(elf_id, calorie_count);
            } else if calorie_count > top3[1].1 {
                top3[2] = top3[1];
                
                top3[1] = assign_elf(elf_id, calorie_count);
            } else if calorie_count > top3[2].1 {
                top3[2] = assign_elf(elf_id, calorie_count);
            }

            calorie_count = 0;
        } else {
            calorie_count = calorie_count + meal.parse::<i32>().unwrap();
        }
    }

    println!("================================");
    println!(
        "The elf carrying the most calories is {} carring {} calories!",
        top3[0].0, top3[0].1
    );
    println!(
        "The elf carrying the second most calories is {} carring {} calories!",
        top3[1].0, top3[1].1
    );
    println!(
        "The elf carrying the third most calories is {} carring {} calories!",
        top3[2].0, top3[2].1
    );
    println!(
        "The elves are carrying {} calories in total.",
        total_calories(top3)
    );
}

fn assign_elf(elf_id:i32, calorie_count: i32) -> (i32, i32) {
    return (elf_id, calorie_count);
}

fn total_calories(top3:[(i32,i32);3]) -> i32 {
    return top3[0].1 + top3[1].1 + top3[2].1;
}
