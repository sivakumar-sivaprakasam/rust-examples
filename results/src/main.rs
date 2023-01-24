fn main() {
    // Declaring variable with Ok() variant of Result type
    let a: Result<i32, &str> = Ok(100);

    // Declaring variable with Err() variant of Result type
    let b: Result<i32, &str> = Err("Invalid input provided");

    // *** Extracting contained value from Option ***

    // is_ok() method returns true/false in case if Result is of Ok() variant
    println!("Is 'a' is of Ok() variant? => {}", a.is_ok());
    println!("Is 'b' is of Ok() variant? => {}", b.is_ok());

    // is_err() method returns true/false in case if Result is of Err() variant
    println!("Is 'a' is of Err() variant? => {}", a.is_err());
    println!("Is 'b' is of Err() variant? => {}", b.is_err());

    // if-let statement
    // Get value of Result type's Ok() variant
    if let Ok(x) = a {
        println!("Extracted value from 'a' is => {x}");
    }

    // Get value of Result type's Err() variant
    if let Err(x) = b {
        println!("Extracted value from 'b' is => {x}");
    }

    // match expression
    match a {
        Ok(x) => println!("Extracted Ok() variant's value from 'a' is => {x}"),
        Err(x) => println!("Extracted Err() variant's value from 'a' is => {x}"),
    }

    match b {
        Ok(x) => println!("Extracted Ok() variant's value from 'b' is => {x}"),
        Err(x) => println!("Extracted Err() variant's value from 'b' is => {x}"),
    }

    // expect() method panics with a provided custom message
    println!("Value of variable a => {}", a.expect("Variable cannot be empty"));

    // unwrap() method panics with a generic message
    println!("Value of variable a => {}", a.unwrap());

    // unwrap_or() returns value of variable OR Ok() variant’s value
    println!("Value of variable a => {}", a.unwrap_or(0));

    // unwrap_or_default() returns value of Ok() variant OR default value of underlying type
    println!("Value of variable a => {}", a.unwrap_or_default());

    // unwrap_or_else() returns value of Ok() variant OR returns the value of the function argument
    println!("Value of variable a => {}", a.unwrap_or_else(|x| x.len() as i32));
    
    // expect() method panics with a provided custom message
    println!("Value of variable a => {}", b.expect_err("Variable cannot be empty"));

    // unwrap() method panics with a generic message
    println!("Value of variable a => {}", b.unwrap_err());

    // Comparing Option
    if a > b {
        println!("Variable value a is greater than value of variable b");
    } else {
        println!("Variable value a is less than value of variable b");
    }

    // Iterating over Result
    let r_arr = [Ok(10), Ok(20), Err("Invalid user input"), Ok(30)];

    // Collecting all the elements from Result array
    let result_1: Result<Vec<i32>, &str> = r_arr.into_iter().collect();
    println!("Collect the elements => {result_1:?}");

    // Filter & collect the elements from Result array
    let result_2: Result<Vec<i32>, &str> = r_arr.into_iter().filter(|x| x.is_ok()).collect();
    println!("Filter & collect the elements => {result_2:?}");

    // Find sum() of all elements from Result array
    let result_3: Result<i32, &str> = r_arr.into_iter().sum();
    println!("Sum of all elements without filter => {result_3:?}");

    // Find product() of all elements from Result array
    let result_4: Result<i32, &str> = r_arr.into_iter().product();
    println!("Product of all elements without filter => {result_4:?}");

    // Find sum() of all elements from Result array
    let result_3: Result<i32, &str> = r_arr.into_iter().filter(|x| x.is_ok()).sum();
    println!("Sum of all elements after filter => {result_3:?}");

    // Find product() of all elements from Result array
    let result_4: Result<i32, &str> = r_arr.into_iter().filter(|x| x.is_ok()).product();
    println!("Product of all elements after filter => {result_4:?}");
}
