use std::{env, fs};

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Unexpected number of arguments!");

    let day = &args[1];
    let input_path = &args[2];

    let input = fs::read_to_string(input_path)
        .expect("Unable to read file!");

    let solution = match day.as_str(){
       "1.1" => day1::part_one(&input).to_string(),
       "1.2" => day1::part_two(&input).to_string(),
       _ => panic!("Unknown day parameter!")
    };

    println!("{solution}");
}


