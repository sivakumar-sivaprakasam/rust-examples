fn main() {
    // Declaring integer array
    let mut i_arr = [1, 2, 3, 4, 5, 6];

    // Getting slice array from main array 
    let i_slice_arr = &mut i_arr[..];

    // Using slice, iterating through all the elements of sliced integers
    i_slice_arr.iter().for_each(|x| print!("{x}, "));

    // Using slice, mutably iterating through the elements and change its values of the sliced integers
    // When we iterate mutably, underlying variable must be declared with mut keyword
    i_slice_arr.iter_mut().map(|x| *x * 2).for_each(|x| print!("{x}, "));

    i_slice_arr.chunks(2).for_each(|x| print!("{x:?}"));

    // it is possible to slice out sub-set of it
    i_slice_arr[2..5].iter().for_each(|x| print!("{x}, "));

    // it is possible to slice out sub-set of it
    i_arr[2..=5].iter().for_each(|x| print!("{x}, "));

    // it will be panic in case if we try to slice beyond the max available items
    //i_arr[..7].iter().for_each(|x| print!("{x}"));

    let s = String::from("Rust is awesome");
    let _ = &s[4..10].chars().filter(|c| c.is_ascii_lowercase()).for_each(|x| print!("{x}, "));
}
