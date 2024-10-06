use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Hello, lets play guess game!");
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is: {}",secret_number);
        println!("Input a guess: ");
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to process.");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
    };
    println!("Guessed is: {}",guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less =>   println!("{}", "Small".red()),
        Ordering::Greater => println!("{}", "Big".red()),
        Ordering::Equal => {
            println!("{}", "You Win!".green());
            break;
        },
    }
}
}
