pub fn luhn(cc_number: &str) -> bool {
    const RADIX:u32 = 10;
    let mut ns: Vec<u32> = vec![];
    for i in cc_number.chars() {
        match i {
            '0'..='9' => ns.push(i.to_digit(RADIX).unwrap()),
            _ => (),
        }
    }
    if ns.len() < 2 {
        return false;
    } else {
        let mut sum_of_digits = 0;
        for (pos, v) in ns.iter().rev().enumerate() {
            let mut t = *v;
            if pos % 2 != 0 {
                t = *v * 2;
                if t > 9 {
                    t -= 9;
                }
            }
            sum_of_digits += t;
        }
        sum_of_digits % 10 == 0
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    let cc_number = " 0 0 ";
    println!(
        "Is {cc_number} a valid credit card number? {}",
        if luhn(cc_number) { "yes" } else { "no" }
    );
}