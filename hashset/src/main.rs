use std::collections::{HashSet, HashMap};

fn main() {
    // Create new HashSet
    let mut hs1: HashSet<_> = HashSet::new();
    hs1.insert("Rust".to_string());
    hs1.insert("is".to_string());
    hs1.insert("awesome".to_string());
    println!("{:?}", hs1);

    // HashSet can also be created from an Array like below
    let hs2: HashSet<&str> = ["Rust", "is", "awesome"].iter().cloned().collect();
    println!("{:?}", hs2);

    // Check the total elements in HashSet
    println!("Length: {}", hs2.len());

    // HashSet can also be created using From trait
    let hs3: HashSet<&str> = HashSet::from(["Rust", "is awesome", "Rust Language"]);
    println!("{:?}", hs3);

    // Creating Hashset of integers
    let hs3: HashSet<i32> = [2, 4, 5, 6, 2, 8].iter().cloned().collect();
    println!("{:?}", hs3);

    // Perform `sum` actions on the values of HashSet
    println!("Sum of all values in HashSet: {}", hs3.iter().sum::<i32>());

    // Perform `product` actions on the values of HashSet
    println!(
        "Product of all values in HashSet: {}",
        hs3.iter().product::<i32>()
    );

    // Another way of performing aggregte operation
    // Perform `fold` actions on the values of HashSet
    // This will multiply each elements of HashSet
    println!(
        "Product of all values in HashSet: {}",
        hs3.iter().fold(1, |x, y| x * y)
    );

    // Create Vector object from HashSet
    // In this example, we use values of HashSet to create new Vector
    let v1: Vec<_> = hs2.iter().collect();
    println!("{:?}", v1);

    // Create new HashMap object from HashSet
    // In this example, we use each element of HashSet as key and its length as value
    let s1: HashMap<_, _> = hs2.iter().map(|x| (x, x.len())).collect();
    println!("{:?}", s1);

    // Apply some filter operations on HashSet
    let v2: Vec<_> = hs2
        .iter()
        .filter(|e| e.len() >= 4)
        .map(|e| e.len())
        .collect();
    println!("{:?}", v2);
}
