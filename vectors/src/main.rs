use std::collections::{HashSet, HashMap};

fn main() {
    // declare new vector
    let i = vec![1, 2, 3, 4, 5];
    println!("Contents of Vector: {:?}", i);

    // another way of creating vector
    let mut b = Vec::new();
    b.push(10);
    b.push(20);
    b.push(30);
    println!("Content of Vector: {:?}", b);

    println!("Poping elements from Vector (LIFO)");
    while let Some(top) = b.pop() {
        println!("{top}");
    }
    
    // apply some transformation on each element of vector
    println!("");
    i.iter().map(|x| x * 2).for_each(|y| print!("{}, ", y));

    // find sum of all elements in a vector
    println!("");
    let s1: i32 = i.iter().sum();
    println!("Sum of all elements in a Vector: {}", s1);

    // find product of all elements in a vector
    println!("");
    let s2: i32 = i.iter().product();
    println!("Product of all elements in a Vector: {}", s2);

    // Merge 2 different vector and create Tuple object
    println!("");
    let j = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let t = i.iter().zip(j.iter());
    t.for_each(|x| print!("{:?} ", x));

    // Create HashMap from a Vector
    println!("");
    // Following operation will create a HashMap with each Vector element as Key and len as Value
    let s3 = vec!["Siva", "Kumar"];    
    let hm: HashMap<_, _> = s3.iter().map(|x| (x, x.len())).collect();
    println!("Created HashMap from a Vector: {:?}", hm);

    // Create HashSet from a Vector
    println!("");
    let hs: HashSet<_> = vec![1, 1, 2, 2, 4, 4, 5, 5, 6, 6].into_iter().collect();
    println!("Created HashSet from a Vector: {:?}", hs);

    
}
