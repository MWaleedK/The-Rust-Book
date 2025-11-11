use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to MWK's guessing game");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Input your number");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("You guessed a smaller number than randomly generated number.")
            }
            Ordering::Equal => {
                println!("You guessed correct!");
                break;
            }
            Ordering::Greater => {
                println!("You guessed a greater number than randomly generated number.")
            }
        }
    }
}
