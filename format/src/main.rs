use num_format::ToFormattedString;

fn main() {
    // Formatting numbers

    // Formatting number
    println!("{}", format!("{}", 100));

    // Formatting number with zero-padded
    println!("{}", format!("{:010}", 5));

    // Format number in binary format
    println!("{}", format!("{:#b}", 50));

    // Format number in octo format
    println!("{}", format!("{:#o}", 50));

    // Format number in hexa-decimal format
    println!("{}", format!("{:#x}", 50));

    // Returns 1000 separator in US format
    println!("{}", 1_000_000_000.to_formatted_string(&num_format::Locale::en));

    // Returns 1000 separator in French format
    println!("{}", 1_000_000_000.to_formatted_string(&num_format::Locale::fr));

    // Returns 1000 separator in Indian format
    println!("{}", 1_000_000_000.to_formatted_string(&num_format::Locale::en_IN));

    // Format decimal numbers
    println!("{}", format!("{:.5}", 100.02));
    
    // Format decimal numbers
    println!("{}", format!("{:.5}", 100.02781839));

    // Format string `Rust is awesome` left-aligned
    println!("{}", format!("{:<20}", "Rust is awesome"));

    // Format string `Rust is awesome` right-aligned
    println!("{}", format!("{:>20}", "Rust is awesome"));

    // Format string `Rust is awesome` center-aligned
    println!("{}", format!("{:^20}", "Rust is awesome"));

    // Format string `Rust is awesome` center-aligned & fill spaces with specific character
    println!("{}", format!("{:*^20}", "Rust is awesome"));

    // Format number right-aligned
    println!("{}", format!("{:>10}", 1000));
}
