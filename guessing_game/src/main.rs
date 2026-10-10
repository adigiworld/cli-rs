use std::{cmp::Ordering, io};

fn main() {
    println!("\n\x1B[1;32m ======================== \x1B[0m");
    println!("\x1B[1;32m Welcome to Guessing Game \x1B[0m");
    println!("\x1B[1;32m ======================== \x1B[0m\n");

    const RAND_NUM: u32 = 37;

    loop {
        println!("Please enter your guess?");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to parse string to number");
                continue;
            }
        };
        // println!("Your guess is : {guess}");
        match guess.cmp(&RAND_NUM) {
            Ordering::Less => println!("\x1B[1;31mToo small\x1B[0m"),
            Ordering::Equal => {
                println!("\n\x1B[1;32m ================ \x1B[0m");
                println!("\x1B[1;32m You won the game \x1B[0m");
                println!("\x1B[1;32m ================ \x1B[0m\n");

                break;
            }
            Ordering::Greater => println!("\x1B[1;31mToo big\x1B[0m"),
        }
    }
}
