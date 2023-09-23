use rand::{self, Rng};
use std::io;
fn main() {
    let mut guessed_number: i32;
    let random_number: i32 = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        println!("Guess the number: ");
        let _ = io::stdin().read_line(&mut guess);
        guessed_number = guess.trim().parse().unwrap();

        if guessed_number > random_number {
            println!("HIGH!")
        } else if guessed_number < random_number {
            println!("LOW!")
        } else {
            println!("CORRECT!");
            break;
        }
    }
}
