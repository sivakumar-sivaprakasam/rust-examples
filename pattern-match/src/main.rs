fn main() {
    // Matching Literals
    let x = 10;

    match x {
        10 => println!("Ten"),
        20 => println!("Twenty"),
        30 => println!("Thirty"),
        _ => println!("Others"),
    }

    // Matching Named Variables
    let x = Some(1);
    let y = 2;

    match x {
        Some(10) => println!("We got the value 10"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("Out of the match construct: x = {:?}, y = {y}", x);

    // Matching Multiple Patters
    let a = 10;

    match a {
        10 | 20 => println!("Ten or Twenty"),
        3 => println!("Thirty"),
        _ => println!("Not in the range"),
    }

    // Matching Range of Values with ..=
    let a = 10;

    match a {
        1..=9 => println!("Between one to nine"),
        10..=20 => println!("Between ten to twenty"),
        _ => println!("Not in the range"),
    }

    // Matching Structs
    let p = Point { x: 0, y: 10 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Matching Enums

    let m = Message::Print(String::from("Hello World"));
    match m {
        Message::Exit => {
            println!("Exit Enum");
        }
        Message::Point { x, y } => {
            println!("Point with x axis {x} and y axis {y}");
        }
        Message::Print(text) => {
            println!("Printing message: {text}");
        }
        Message::RGB(r, g, b) => {
            println!("Red {r}, Green {g}, and Blue {b}",)
        }
    }

    // Ignoring entire value using _

    let a = 5;
    let b = 10;

    match (a, b) {
        (_, 10) => println!("Ignored a but matched b which is having value 10"),
        _ => println!("All othe cases")
    } 

    // Ignoring Remaining Parts of a Value with ..

    let x = 1;
    let y = 2;
    let z = 3;

    match (x, y, z) {
        (1, ..) => println!("x is 1 but ignored the other values"),
        _ => println!("All other cases")
    }

    // Extra Conditionals with Match Guards

    let number = 50;

    match number {
        x if x % 10 == 0 => println!("{x} is divisable by 10"),
        x if x % 3 == 0 => println!("{x} is divisable by 3"),
        _ => println!("All other cases")
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Exit,
    Point { x: i32, y: i32 },
    Print(String),
    RGB (i32, i32, i32),
}