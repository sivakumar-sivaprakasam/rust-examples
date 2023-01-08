use std::collections::{BTreeMap, HashSet};

fn main() {
    // Create new BTreeMap
    let mut bm1: BTreeMap<_, _> = BTreeMap::new();
    bm1.insert("Rust".to_string(), 4);
    bm1.insert("is awesome".to_string(), 5);
    println!("Contents of BTreeMap created by using new() method {:?}", bm1);

    // BTreeMap can also be created from an Array like below
    let bm2: BTreeMap<&str, i32> = [("Rust", 4), ("is awesome", 5), ("Rust Language", 4)].iter().cloned().collect();
    println!("Contents of BTreeMap created by using arrays {:?}", bm2);

    // Check the total elements in BTreeMap
    println!("Length: {}", bm2.len());

    // HashMap can be created by using vec! macro as well
    let bm3: BTreeMap<&str, i32> = vec![("Rust", 4), ("is awesome", 5), ("Rust Language", 4)].into_iter().collect();
    println!("Contents of BTreeMap created by using vec! macro {:?}", bm3);

    // Perform `sum` actions on the values of BTreeMap
    // This will add each elements' value of BTreeMap
    println!("Sum of all values in BTreeMap: {}", bm2.values().sum::<i32>());

    // Perform `product` actions on the values of BTreeMap
    // This will multiply each elements' value of BTreeMap
    println!("Product of all values in BTreeMap: {}", bm2.values().product::<i32>());

    // Another way of performing aggregte operation
    // Perform `fold` actions on the values of BTreeMap
    // This will multiply each elements of BTreeMap
    println!("Product of all values in BTreeMap using fold method: {}", bm2.values().fold(1, |x, y| x*y));

    // Create Vector object from BTreeMap
    // In this example, we use values of BTreeMap to create new Vector
    let v1: Vec<_> = bm2.values().collect();
    println!("Contents of Vector created out from BTreeMap {:?}", v1);

    // Create HashSet object from BTreeMap
    // In this example, we use values of BTreeMap to create new HashSet
    let s1: HashSet<_> = bm2.values().collect();
    println!("Contents of HashSet created out from BTreeMap {:?}", s1);

    // Apply some filter operations on BTreeMap
    // Following example filter elements' Key equals Rust and 
    // return only that element's value
    let v2: Vec<_> = bm2.into_iter().filter(|e| e.0 == "Rust").map(|e| e.1).collect();
    println!("Filtered Vector from BTreeMap {:?}", v2);
}
