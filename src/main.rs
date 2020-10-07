use std::cmp::Ordering;
use std::io::stdin;

use rand::Rng;

fn main() {
    println!("Guess the number");
    let mut guess;
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess");
        guess = String::new();
        stdin().read_line(&mut guess).expect("Fail to read line");
        println!("You guessed {}", guess);
        /**/
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
        println!("The secret number is {}", secret_number);
    }
}
