use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess.");
        let mut guess = String::new();
        // var is immutable by default
        io::stdin()
            .read_line(&mut guess) // this is an append, mut is needed for it to be modified
            .expect("Failed to read line"); // last line returns a Result, Ok or Err, that has expect method
                                            // If Ok, return the value
                                            // If Err, print msg
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
    
    let x = 5;
    let y = 10;
    println!("x = {x}, y+3 = {}", y + 3);
}
