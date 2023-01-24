fn main() -> Result<(), String> {
    // Declaring variable with Ok() variant of Result type
    let a: Result<i32, &str> = Ok(100);

    // expect() method panics with a provided custom message
    println!("Value of variable a => {}", a.expect("Variable cannot be empty"));

    // unwrap() method panics with a generic message
    println!("Value of variable a => {}", a.unwrap());

    // When there is no ? operator to propagate errors
    let r = print_me(2);
    if let Err(x) = r {
        return Err(x)
    }

    let r = print_me(3);
    if let Err(x) = r {
        return Err(x)
    }

    // Using ? operator to propagate errors
    print_me(100)?;
    return Ok(())
}

fn print_me(n: i32) -> Result<i32, String> {
    if n < 5 {
        Ok(n)
    } else {
        Err(format!("Given input {n} is invalid!!!"))
    }
}