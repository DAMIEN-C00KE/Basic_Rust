// Simple program that converts a decimal number to its binary, and hexidecimal equivalent.

use std::io;

fn main() {

    println!("Enter a decimal number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid decimal number.");
            return;
        }
    };

    println!("Binary: {}", to_binary(number));
    println!("Hexadecimal: {}", to_hexadecimal(number));
}

fn to_binary(number: u32) -> String {
    format!("{:b}", number)
}

fn to_hexadecimal(number: u32) -> String {
    format!("{:x}", number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_binary() {
        assert_eq!(to_binary(28), "11100");
        assert_eq!(to_binary(255), "11111111");
        assert_eq!(to_binary(0), "0");
    }

    #[test]
    fn test_to_hexadecimal() {
        assert_eq!(to_hexadecimal(28), "1c");
        assert_eq!(to_hexadecimal(255), "ff");
        assert_eq!(to_hexadecimal(0), "0");
    }
}
