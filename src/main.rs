use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to my guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please enter a number below");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read the contents of the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You have guesed {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("equal");
                break;
            },
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
        }
    }
    

}
