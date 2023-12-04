use std::io;
use rand::{self, thread_rng, Rng};

pub fn guessing_game(){

    let secret = thread_rng().gen_range(1..=10);

    loop{
        println!("Please guess a number between 1-10: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Oop, input error.");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("You didn't enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
    }
}
