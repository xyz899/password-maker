use std::io;
use std::io::Write;


pub fn password_maker() -> String {
    let chars = "abcdefghijklmopqrstuvwxyz";
    let upper_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let special = "@#&é'(§è!çà)-_{}";
    let numbers = "1234567890";
    let combined = format!("{}{}{}{}", chars, upper_chars, numbers, special);

    print!("Choose a length: ");
    io::stdout().flush().unwrap(); 

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let length: usize = choice.trim().parse().unwrap_or(8); // Default to 8 if parsing fails

    let mut password = String::new();
    for _ in 0..length {
        let idx = rand::random::<usize>() % combined.len();
        password.push(combined.chars().nth(idx).unwrap());
    }

    password
}

fn main() {
    let password = password_maker();
    println!("Generated password: {}", password);
}
