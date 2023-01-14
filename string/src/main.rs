fn main() {
    // Creating String by various ways

    // Using String struct's from method
    let s1 = String::from("Rust is awesome");
    println!("{s1}");

    // It is possible to create String from unicode characters
    let s2 = String::from("铁锈很棒");
    println!("{s2}");

    // Using String struct's from_utf8 method by passing vec of u8
    let s3 = String::from_utf8(vec![97, 98, 99, 100, 101, 102]).unwrap();
    println!("{s3}");

    // Convert str literal to String using to_string() method
    let s4 = "Rust is awesome".to_string();
    println!("{s4}");

    // Creating String from char array & Iterator trait's collect method
    let c_arr = ['R', 'u', 's', 't', ' ', 'i', 's', ' ', 'a', 'w', 'e', 's', 'o', 'm', 'e'];
    let s5 = c_arr.iter().map(|c| c.to_ascii_uppercase()).collect::<String>();
    println!("{s5}");

    // Creating string literal from String reference
    let str1 = &s1;
    println!("{str1}");

    // Creating string slice from literal String
    let str2 = "Rust is awesome";
    println!("{str2}");

    // Creating string slice from String's as_str method
    let str3 = s4.as_str();
    println!("{str3}");

    // Comparing string slice with String reference
    println!("'{str1}' == '{str2}' => {}", str1 == str2);

    // Comparing String with string slice 
    println!("'{}' == '{}' => {}", str2, s1, str2 == s1);

    print_str_and_its_len(str1);
    print_str_and_its_len(&s4[0..]);
}

fn print_str_and_its_len(s: &str) {
    println!("string = {}, length = {}", s, s.len());
}