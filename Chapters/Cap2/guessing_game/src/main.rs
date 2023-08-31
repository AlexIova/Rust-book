use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);

    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
