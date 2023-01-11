use std::collections::{BinaryHeap, HashMap};

fn main() {
    // Create new BinaryHeap (Max-Heap)
    let mut bh = BinaryHeap::new();
    bh.push(10);
    bh.push(20);
    bh.push(30);
    bh.push(40);
    bh.push(50);

    // print doesnt guarantee the order of items
    println!("");
    println!("Contents of BinaryHeap (max-heap): {:?}", bh);

    // Creating BinaryHeap (Max-Heap) using From trait
    let bh1 = BinaryHeap::from([1, 2, 3, 4, 5]);

    // print doesnt guarantee the order of items
    println!("");
    println!(
        "Contents of BinaryHeap (max-heap) created using From trait: {:?}",
        bh1
    );

    // Creating BinaryHeap (Max-Heap) using IntoInterator trait
    let bh2: BinaryHeap<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();

    // print doesnt guarantee the order of items
    println!("");
    println!(
        "Contents of BinaryHeap (max-heap) created using IntoIterator trait: {:?}",
        bh2
    );

    // iterator doesnt guarantee the order of items
    println!("");
    println!("Iterating over BinaryHeap (max-heap):");
    for x in &bh {
        println!("{x}");
    }

    // Peek method used to get greatest item in the BinaryHeap (Max-Heap)
    println!("");
    if let Some(t) = bh.peek() {
        println!("Greatest item from BinaryHeap (max-heap): {}", t);
    }

    // pop method returns items from max to min items
    println!("");
    println!("Loop through BinaryHeap (max-heap) and retrieve element using pop method:");
    while let Some(top) = bh.pop() {
        println!("Retrieving element using pop() method from BinaryHeap (max-heap): {top}");
    }

    // Create new BinaryHeap (Min-Heap)
    let mut bh3 = BinaryHeap::new();
    bh3.push(core::cmp::Reverse(10));
    bh3.push(core::cmp::Reverse(20));
    bh3.push(core::cmp::Reverse(30));
    bh3.push(core::cmp::Reverse(40));
    bh3.push(core::cmp::Reverse(50));

    // print doesnt guarantee the order of items
    println!("");
    println!("Contents of BinaryHeap (min-heap): {:?}", bh3);

    // Creating BinaryHeap (Min-Heap) using From trait
    let bh4 = BinaryHeap::from([
        core::cmp::Reverse(1),
        core::cmp::Reverse(2),
        core::cmp::Reverse(3),
        core::cmp::Reverse(4),
        core::cmp::Reverse(5),
    ]);

    // print doesnt guarantee the order of items
    println!("");
    println!(
        "Contents of BinaryHeap (min-heap) created using From trait: {:?}",
        bh4
    );

    // Creating BinaryHeap (Min-Heap) using IntoInterator trait
    let bh5: BinaryHeap<core::cmp::Reverse<i32>> = vec![
        core::cmp::Reverse(1),
        core::cmp::Reverse(2),
        core::cmp::Reverse(3),
        core::cmp::Reverse(4),
        core::cmp::Reverse(5),
    ]
    .into_iter()
    .collect();

    // print doesnt guarantee the order of items
    println!("");
    println!(
        "Contents of BinaryHeap (min-heap) created using IntoIterator trait: {:?}",
        bh5
    );

    // iterator doesnt guarantee the order of items
    println!("");
    println!("Iterating over BinaryHeap (min-heap):");
    for x in &bh5 {
        println!("Element from BinaryHeap (min-heap) {:?}", x);
    }

    // Peek method used to get greatest item in the BinaryHeap (Min-Heap)
    println!("");
    if let Some(t) = bh3.peek() {
        println!("Greatest item from BinaryHeap (min-heap): {:?}", t);
    }

    // pop method returns items from min to max items
    println!("");
    println!("Loop through BinaryHeap and retrieve element using pop method:");
    while let Some(top) = bh3.pop() {
        println!(
            "Retrieving element using pop() method from BinaryHeap (min-heap): {:?}",
            top
        );
    }

    // Finding sum of all elements
    println!("");
    println!("Total elements from BinaryHeap: {}", bh1.iter().count());
    println!(
        "Sum of all elements from BinaryHeap: {}",
        bh1.iter().sum::<i32>()
    );

    // Finding product of all elements
    println!(
        "Product of all elements from BinaryHeap: {}",
        bh1.iter().product::<i32>()
    );

    // Merge 2 different vector and create Tuple object
    println!("");
    let j = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let t = bh1.iter().zip(j.iter());
    t.for_each(|x| println!("Zipped Vector with BinaryHeap: {:?} ", x));

    // Create HashMap out from a VecDeque object

    // Following operation will create a HashMap with each VecDeque element as Key and len as Value
    println!("");
    let mut s: BinaryHeap<&str> = BinaryHeap::new();
    s.push("Rust");
    s.push("is");
    s.push("awesome");
    let hm1: HashMap<_, _> = s.iter().map(|x| (x, x.len())).collect();
    println!("Created HashMap out from BinaryHeap: {:?}", hm1);
}
