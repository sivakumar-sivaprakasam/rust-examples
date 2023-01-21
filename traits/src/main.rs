fn main() {
    let s: String = String::from("Rust");
    let i = 100;
    let s1 = "Rust is awesome";
    let e = Employee::new(String::from("John"), 30);
    println!("Is '{s}' within range?: {}", s.is_in_range());
    println!("Is '{i}' within range?: {}", i.is_in_range());
    println!("Is '{s1}' within range?: {}", s1.is_in_range());
    println!("Is {e:#?} within range?: {}", e.is_in_range());
}

trait TypeUtils {
    fn is_in_range(&self) -> bool;
}

impl TypeUtils for String {
    fn is_in_range(&self) -> bool {
        false
    }
}

impl TypeUtils for i32 {
    fn is_in_range(&self) -> bool {
        true
    }
}

impl TypeUtils for str {
    fn is_in_range(&self) -> bool {
        true
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Employee {
    name: String,
    age: i32,
}

impl Employee {
    fn new(name: String, age: i32) -> Self {
        Employee { name, age }
    }
}

impl TypeUtils for Employee {
    fn is_in_range(&self) -> bool {
        true
    }
}