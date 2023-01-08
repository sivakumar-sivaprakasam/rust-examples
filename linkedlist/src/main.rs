use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // LinkedList can be created using various ways

    // Create an empty LinkedList
    let a: LinkedList<i32> = LinkedList::new();
    println!("Contents of LinkedList a: {:?}", a);

    // Create LinkedList with known set of values
    let b: LinkedList<i32> = LinkedList::from([1, 2, 3, 4, 5]);
    println!("Contents of LinkedList b: {:?}", b);

    // LinkedList can be created from Vector as below
    let ll: LinkedList<_> = vec![1, 2, 3, 4, 5].into_iter().collect();
    println!("Contents of LinkedList l1: {:?}", ll);

    // LinkedList can be created as below as well
    let mut i: LinkedList<i32> = LinkedList::new();
    i.push_front(1);
    i.push_front(2);
    i.push_front(3);
    i.push_front(4);
    i.push_front(5);
    println!("Contents of LinkedList i: {:?}", i);

    // apply some transformation on each element of LinkedList
    i.iter().map(|x| x*2).for_each(|y| println!("{}, ", y));

    // find sum of all elements in a LinkedList
    let s1: i32 = i.iter().sum();
    println!("Sum of all elements in LinkedList i: {}", s1);

    // find product of all elements in a LinkedList
    let s2: i32 = i.iter().product();
    println!("Product of all elements in LinkedList i: {}", s2);

    // Create HashMap out from a LinkedList
    // Following operation will create a HashMap with each LinkedList element as Key and len as Value
    let s3: LinkedList<_> = vec!["Rust", "is", "awesome"].into_iter().collect();    
    let hm1: HashMap<_, _> = s3.iter().map(|x| (x, x.len())).collect();
    println!("HashMap -> {:?}", hm1);

    // Create HashSet from a LinkedList
    let s4: LinkedList<_> = vec![1, 1, 2, 2, 4, 4, 5, 5, 6, 6].into_iter().collect();
    let hs1: HashSet<_> = s4.iter().collect();
    println!("HashSet -> {:?}", hs1);
}
