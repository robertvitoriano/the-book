use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game started");
    
    println!("Please enter a guess:");

    let mut is_playing = true;

    enum PlayAgain {
        Yes,
        No,
    }

    impl PlayAgain {
        fn from_str(input: &str) -> Option<PlayAgain> {
            match input.trim().to_lowercase().as_str() {
                "s" => Some(PlayAgain::Yes),
                "n" => Some(PlayAgain::No),
                _ => None,
            }
        }
    }

    while is_playing {
        
        let secret_number = rand::thread_rng().gen_range(1, 101);

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
                Ordering::Equal =>{
                     println!("You guessed it, the number is {}", secret_number);
                     break;
                    },
            }
            
        }
        
        print!("Do you wanna play again ? s/n");
        
        let mut play_again_answer = String::new();

        io::stdin()
            .read_line(&mut play_again_answer)
            .expect("failed to read user input");
        
        match PlayAgain::from_str(&play_again_answer) {
            Some(PlayAgain::Yes) => {
                is_playing = true;
            }
            Some(PlayAgain::No) => {
                is_playing = false;
            }
            None => {
                println!("Invalid input, please enter 's' or 'n'.");
            }
        }
    }
}
