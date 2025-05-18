use std::io;

fn main() {
    println!("Gussing game started");
    
    let mut user_guess = String::new();
    
    io::stdin().read_line(&mut user_guess)
        .expect("Failed to read line");

    }
