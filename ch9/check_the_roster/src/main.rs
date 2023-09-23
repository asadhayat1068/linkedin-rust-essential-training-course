use std::env;
use std::fs;

fn main() {
    if env::args().len() < 3 {
        println!("Program require two arguments. <path> <name>");
        std::process::exit(1);
    }
    let file_path = env::args().nth(1).unwrap();
    let person_to_check = env::args().nth(2).unwrap();
    let person_list = fs::read_to_string(file_path).unwrap();
    for person in person_list.lines() {
        if person == person_to_check {
            println!("{} found in the list!", person);
            return;
        }
    }
    println!("Person not found in the list");
}
