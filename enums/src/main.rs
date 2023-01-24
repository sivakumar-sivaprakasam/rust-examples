#![allow(dead_code)]
fn main() {
    let a = Ack::ExactlyOnce;
    println!("Received acknowledge {a:#?}");

    let b = EnhancedAck::ExactlyOnce;
    println!("Received Acknowledge: {b:?}");

    let c = EnhancedAck::AtMostOnce(String::from("Hello Rust"));
    println!("Received Acknowledge: {c:?}");
    if let EnhancedAck::AtMostOnce(c1) = c {
        println!("Extracted data from Enum --> Message: {}", c1);
    }

    let d = EnhancedAck::AtLeastOnce(100, String::from("Hello Rust Again"));
    println!("Received Acknowledge: {d:?}");
    if let EnhancedAck::AtLeastOnce(d1, d2) = d {
        println!("Extracted data from Enum --> Number: {}, Message: {}", d1, d2);
    }
}

#[derive(Debug)]
enum Ack {
    ExactlyOnce,
    AtMostOnce,
    AtLeastOnce
}

#[derive(Debug)]
enum EnhancedAck {
    ExactlyOnce,
    AtMostOnce(String),
    AtLeastOnce(i32, String)
}