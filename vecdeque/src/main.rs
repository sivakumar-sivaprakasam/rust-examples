use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    // VecDeque can be created in many ways

    // Create empty VecDeque
    let a: VecDeque<i32> = VecDeque::new();
    println!("Contents of VecDeque {:?}", a);

    // Create VecDeque with set of values
    let b: VecDeque<i32> = VecDeque::from([1, 2, 3, 4, 5]);
    println!("Contents of VecDeque {:?}", b);

    // VecDeque can be created from Vector
    let c: VecDeque<_> = vec![1, 2, 3, 4, 5].into_iter().collect();
    println!("Contents of VecDeque {:?}", c);

    // VecDeque can be created and later populated with values
    // Using push_back() and push_front() methods
    let mut i: VecDeque<i32> = VecDeque::new();
    i.push_back(1);
    i.push_front(10);
    i.push_back(2);
    i.push_front(20);
    i.push_back(3);
    i.push_front(30);
    i.push_back(4);
    i.push_front(40);
    i.push_back(5);
    i.push_front(50);
    println!("Contents of VecDeque from rear-end {:?}", i);
    println!("Poping elements from VecDeque (front): ");
    if let Some(top) = i.pop_front() {
        println!("{top}");
    }
    println!("Poping elements from VecDeque (back): ");
    if let Some(top) = i.pop_back() {
        println!("{top}");
    }
    println!("Contents of VecDeque from front-end {:?}", i);
    println!("Poping elements from VecDeque (front): ");
    if let Some(top) = i.pop_front() {
        println!("{top}");
    }
    println!("Poping elements from VecDeque (back): ");
    if let Some(top) = i.pop_back() {
        println!("{top}");
    }

    println!("Contents of VecDeque from front-end {:?}", i);

    // apply some transformation on each element of VecDeque
    i.iter().map(|x| x * 2).for_each(|y| println!("{}, ", y));

    // find sum of all elements in a VecDeque
    let s1: i32 = i.iter().sum();
    println!("Sum of all elements from VecDeque: {}", s1);

    // find product of all elements in a VecDeque
    let s2: i32 = i.iter().product();
    println!("Product of all elements from VecDeque: {}", s2);

    // Merge 2 different VecDeque and create Tuple object
    let j: VecDeque<_> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1].into_iter().collect();
    let t = i.iter().zip(j.iter());
    t.for_each(|x| println!("{:?} ", x));

    // Create HashMap out from a VecDeque object
    // Following operation will create a HashMap with each VecDeque element as Key and len as Value
    let s3: VecDeque<_> = vec!["Rust", "is", "awesome"].into_iter().collect();
    let hm1: HashMap<_, _> = s3.iter().map(|x| (x, x.len())).collect();
    println!("{:?}", hm1);
}
