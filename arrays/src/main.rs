fn main() {
    // create & initialize array with default values
    let i_arr: [i32; 5] = [0; 5];

    // print content of array
    println!("{:?}", i_arr);
    println!("Length: {}", i_arr.len());

    // Iterating using .iter() method
    for i in i_arr.iter() {
        println!("{} ", i);
    }

    // create & initialize array
    let a_arr = [10, 20, 30, 40, 50];

    // print the array elements in reverse order
    for i in a_arr.iter().rev() {
        println!("{} ", i);
    }

    let mut c_arr: [i32; 5] = [30, 20, 10, 40, 50];
    
    // sort an array
    c_arr.sort();

    println!("Sorted Array: {:?}", c_arr);

    println!("Before swap: {:?}", c_arr);
    c_arr.swap(3, 4);
    println!("After swap: {:?}", c_arr);

}