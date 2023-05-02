use regex::Regex;

fn main() {
    // Parsing Integers
    let re_int = Regex::new(r"\d").unwrap();
    println!("Is 100 is valid integer? {}", re_int.is_match("100"));
    println!("Is HelloWorld is valid integer? {}", re_int.is_match("HelloWorld"));

    // Parsing Strings
    let re_str = Regex::new(r"[[:alpha]]").unwrap();
    println!("Is 100 is valid alphabet? {}", re_str.is_match("100"));
    println!("Is HelloWorld is valid alphabet? {}", re_str.is_match("HelloWorld"));

    // Parsing Date
    let re_ymd = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Is 2023-01-01 matching against YYYY-MM-DD? {}", re_ymd.is_match("2023-01-01"));
    println!("Is 01-01-2023 matching against YYYY-MM-DD? {}", re_ymd.is_match("01-01-2023"));
}
