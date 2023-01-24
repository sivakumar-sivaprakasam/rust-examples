fn main() {
    // Declaring variable with None
    let a: Option<i32> = None;
    println!("Variable has got a value? {}", a.is_some());

    // Declaring variable with Some(100)
    let b: Option<i32> = Some(100);
    println!("Variable has got a value? {}", b.is_some());

    // *** Extracting contained value from Option ***

    // Using if let statement to retrieve value 
    if let Some(x) = a {
        println!("Value of variable a => {x}");
    }

    // match expression is another way to retrieve value
    match b {
        Some(x) => println!("Value of variable b => {x}"),
        None => println!("Value of variable b => None"),
    }

    // expect() method panics with a provided custom message
    println!("Value of variable a => {}", b.expect("Variable cannot be empty"));

    // unwrap() method panics with a generic message
    println!("Value of variable a => {}", b.unwrap());

    // unwrap_or() returns value of variable OR specified value
    println!("Value of variable a => {}", a.unwrap_or(0));

    // unwrap_or_default() returns value of variable OR default value of underlying type
    println!("Value of variable a => {}", a.unwrap_or_default());

    // unwrap_or_else() returns value of variable OR returns the value of the function argument
    println!("Value of variable a => {}", a.unwrap_or_else(|| 10));

    // Comparing Option

    if a > b {
        println!("Variable value a is greater than value of variable b");
    } else {
        println!("Variable value a is less than value of variable b");
    }

    // Iterating over Option
    let o_arr = [Some(10), Some(20), None, Some(30)];

    // Collecting all the elements from Option array
    let result_1: Option<Vec<i32>> = o_arr.into_iter().collect();
    println!("Collect the elements => {result_1:?}");

    // Filter & collect the elements from Option array
    let result_2: Option<Vec<i32>> = o_arr.into_iter().filter(|x| x.is_some()).collect();
    println!("Filter & collect the elements => {result_2:?}");

    // Find sum() of all elements from Option array
    let result_3: Option<i32> = o_arr.into_iter().sum();
    println!("Sum of all elements without filter => {result_3:?}");

    // Find product() of all elements from Option array
    let result_4: Option<i32> = o_arr.into_iter().product();
    println!("Product of all elements without filter => {result_4:?}");

    // Find sum() of all elements from Option array
    let result_3: Option<i32> = o_arr.into_iter().filter(|x| x.is_some()).sum();
    println!("Sum of all elements after filter => {result_3:?}");

    // Find product() of all elements from Option array
    let result_4: Option<i32> = o_arr.into_iter().filter(|x| x.is_some()).product();
    println!("Product of all elements after filter => {result_4:?}");
}
