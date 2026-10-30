use std::{cmp::Ordering, io};
use rand::RngExt;

//猜数字游戏例子
fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut input: String = String::new();
        let bytes_read: usize = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if bytes_read == 0 {
            println!("Input closed. Goodbye!");
            break;
        }

        let guess: u32 = match input
            .trim()
            .parse()
        {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
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