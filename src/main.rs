use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number game! Made in Rust, heheh");

    let mut guesses: u32 = 0;
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Secret Number: {secret_number}");

    loop{        
        println!("Input a number: ");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Error on read number");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        guesses += 1;
    
        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("You won!");
                break;
            },
            Ordering::Greater => println!("A little bit less"),
            Ordering::Less => println!("A litte bit more"),
        }
    }

    println!("You've tried {guesses} times")
}
