use std::collections::{HashMap, HashSet};

fn main() {
// Create new HashMap
    let mut hm1: HashMap<_, _> = HashMap::new();
    hm1.insert("Rust".to_string(), 4);
    hm1.insert("is awesome".to_string(), 5);
    println!("Contents of HashMap created by using new() method {:?}", hm1);

    // HashMap can also be created from an Array like below
    let hm2: HashMap<&str, i32> = [("Rust", 4), ("is awesome", 5), ("Rust Language", 4)].iter().cloned().collect();
    println!("Contents of HashMap created by using arrays {:?}", hm2);

    // Check the total elements in HashMap
    println!("Length: {}", hm2.len());

    // HashMap can be created by using vec! macro as well
    let hm3: HashMap<&str, i32> = vec![("Rust", 4), ("is awesome", 5), ("Rust Language", 4)].into_iter().collect();
    println!("Contents of HashMap created by using vec! macro {:?}", hm3);

    // Perform `sum` actions on the values of HashMap
    // This will add each elements' value of HashMap
    println!("Sum of all values in HashMap: {}", hm2.values().sum::<i32>());

    // Perform `product` actions on the values of HashMap
    // This will multiply each elements' value of HashMap
    println!("Product of all values in HashMap: {}", hm2.values().product::<i32>());

    // Another way of performing aggregte operation
    // Perform `fold` actions on the values of HashMap
    // This will multiply each elements of HashMap
    println!("Product of all values in HashMap using fold method: {}", hm2.values().fold(1, |x, y| x*y));

    // Create Vector object from HashMap
    // In this example, we use values of HashMap to create new Vector
    let v1: Vec<_> = hm2.values().collect();
    println!("Contents of Vector created out from HashMap {:?}", v1);

    // Create HashSet object from HashMap
    // In this example, we use values of HashMap to create new HashSet
    let s1: HashSet<_> = hm2.values().collect();
    println!("Contents of HashSet created out from HashMap {:?}", s1);

    // Apply some filter operations on HashMap
    // Following example filter elements' Key stars_with Rust and 
    // return only that element's value
    let v2: Vec<_> = hm2.into_iter().filter(|e| e.0.starts_with("Rust")).map(|e| e.1).collect();
    println!("Filtered Vector from HashMap {:?}", v2);
}
