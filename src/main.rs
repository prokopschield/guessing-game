use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");

    let secret_number: i64 = rand::thread_rng().gen_range(-1, 1000);
    let mut attempts: u64 = 0;

    loop {
        print!("Guess >> ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Your guess was too small."),
            std::cmp::Ordering::Greater => println!("Your guess was too large."),
            std::cmp::Ordering::Equal => {
                println!("You won after {} wrong attempts!", attempts);
                println!("Congratulations!");
                break;
            }
        }

        attempts += 1;
    }
}
