fn main() {
    println!("Hello, world!");
    println!("Hello from Rust!");
    let mut age: i32 = 18;
    println!("My age now: {}", age);
    age = 19;
    println!("My age will be {} on october 19th.",age);
    let num: f64 = 124.452;
    println!("If {} devided by 2 will be equal to: {}", num, num / 2.0);
    let mut name: &str = "Muhammadazimxon"; // unsafe
    println!("My name is {}.\nNice to meet you!", name);
    let mut surname = String::from("Jamoliddinov"); // safe for use always
    println!("My surname is {}", surname);
    let mut symbole: char = 's';
    println!("{}", symbole);
    let mut is_true: bool = true;
    let num = if is_true {
        1
    } else {
        0
    };
    println!("{}", num);
}
