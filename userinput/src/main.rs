use std::io;

fn main() {
    let stdin = io::stdin();
    println!("Enter your name: ");
    let mut uname = String::new();
    stdin.read_line(&mut uname).unwrap();

    println!("Enter your date of birth (YYYY-MM-DD): ");
    let mut udob: String = String::new();
    stdin.read_line(&mut udob).unwrap();

    println!("Hi! {}, you're {} years old", uname.trim(), udob.trim());
}
