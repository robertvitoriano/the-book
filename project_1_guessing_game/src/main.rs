use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game started");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("Please enter a guess:");
    
    loop {
        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: u32 = user_guess
            .trim()
            .parse()
            .expect("Failed parsing user guess");

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You guessed it, the number is {}", secret_number),
        }
    }
}
