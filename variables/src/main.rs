use std::io;
use rand::Rng;
use colored::*;
fn main() {
    let mut tup = ("Here is the string.", 244);
    let (text, number) = tup;
    let number = tup.1; // tup.1 means access it index number to a new variable

    let error_codes = [200, 400, 500];
    let not_found = error_codes[1];

    //let byte: = [0; 8] // [0; 8] means create an array with length 8 and only elemnts 0


    let sum = my_function(51, 14); // calling function  
    println!("The sum is: {}", sum);

    // control Flow
    let mut input_number: String = String::new();
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 100);
        println!("Secret-number is: {}", secret_number);
        println!("Please input only numbers to compare it with secret number: ");
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to process!");
    let number: i32 = match input_number.trim().parse() {
        Ok(input_number) => input_number,
        Err(_) => break
    };
    if number == secret_number {
        println!("{}", "You win!".green());
        break; // also we can use break secretnumber or number to break and return!
    } else {
        println!("{}", "Secret-number didn't match with number!".red());
        continue;
    }
}
// loops
let mut a = 3;
while a > 0 {
    a -= 1;
    println!("{}", "Works!!!".green());
}
let mut a = [1, 2, 3];
for element in a.iter() {
    println!("{} this is array element.", element);
}
for number in 1..5 { // starting from 1 to 4 will be printed
    print!("{}", number);
}
}
fn my_function(x: i32, y: i32) -> i32 { // fn name always in lower case and '->' means return type
    println!("This is another function.");
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
    return x + y;
}
