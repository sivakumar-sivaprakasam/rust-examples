use std::collections::{BTreeSet, HashMap};

fn main() {
    // Create new BTreeSet
    let mut bt1: BTreeSet<_> = BTreeSet::new();
    bt1.insert("Rust".to_string());
    bt1.insert("is".to_string());
    bt1.insert("awesome".to_string());
    println!("{:?}", bt1);

    // BTreeSet can also be created from an Array like below
    let bt2: BTreeSet<&str> = ["Rust", "is", "awesome"].iter().cloned().collect();
    println!("{:?}", bt2);

    // Check the total elements in BTreeSet
    println!("Length: {}", bt2.len());

    // BTreeSet can also be created using From trait
    let hs3: BTreeSet<&str> = BTreeSet::from(["Rust", "is awesome", "Rust Language"]);
    println!("{:?}", hs3);

    // Creating BTreeSet of integers
    let bt3: BTreeSet<i32> = [2, 4, 5, 6, 2, 8].iter().cloned().collect();
    println!("{:?}", bt3);

    // Perform `sum` actions on the values of BTreeSet
    println!("Sum of all values in BTreeSet: {}", bt3.iter().sum::<i32>());

    // Perform `product` actions on the values of BTreeSet
    println!(
        "Product of all values in BTreeSet: {}",
        bt3.iter().product::<i32>()
    );

    // Another way of performing aggregte operation
    // Perform `fold` actions on the values of BTreeSet
    // This will multiply each elements of BTreeSet
    println!(
        "Product of all values in BTreeSet: {}",
        bt3.iter().fold(1, |x, y| x * y)
    );

    // Create Vector object from BTreeSet
    // In this example, we use values of BTreeSet to create new Vector
    let v1: Vec<_> = bt2.iter().collect();
    println!("{:?}", v1);

    // Create new HashMap object from BTreeSet
    // In this example, we use each element of BTreeSet as key and its length as value
    let s1: HashMap<_, _> = bt2.iter().map(|x| (x, x.len())).collect();
    println!("{:?}", s1);

    // Apply some filter operations on BTreeSet
    let v2: Vec<_> = bt2
        .iter()
        .filter(|e| e.len() >= 4)
        .map(|e| e.len())
        .collect();
    println!("{:?}", v2);
}
