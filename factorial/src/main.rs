// Problem 1: Write a program which finds the factorial of a number entered by the user. (check for all conditions).

use std::io;


fn main() {
    let given = take_user_input();
    match factorial(given) {
        Some(output) => { println!("{given}! = {output}"); },
        None => { println!("Output too large")}
    }
}

// Naive implementation
fn factorial(x: u32) -> Option<u128> {
    if x == 0 {
        return Some(1)
    }
    
    let mut product = 1u128;
    for i in 1u128..=x.into() {
        product = match product.checked_mul(i) {
            Some(it) => it,
            None => return None,
        }
    }
    Some(product)
}

fn take_user_input() -> u32 {
    const BUFFER_CAPACITY: usize = 32;
    let mut buffer = String::with_capacity(BUFFER_CAPACITY); 

    loop { //Loop until correct input is given
        println!("Enter positive integer");
        // io Error handling
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {},
            Err(error) => {
                buffer = String::with_capacity(BUFFER_CAPACITY);
                println!("Error: {}\n Please try again", error)
            }
        }

        // Parsing
        match buffer.trim().parse::<u32>() {
            Ok(given) => { return given }
            Err(_) => { println!("Input must be a positive integer. Try again.")}
        }
    }
} 

#[cfg(test)]
mod factorial_tests {
    use super::*;

    #[test]
    fn factorial_0() { assert_eq!(factorial(0), Some(1)); }

    #[test]
    fn factorial_1() { assert_eq!(factorial(1), Some(1));}
    
    #[test]
    fn factorial_n() {
        let inputs = [2, 3, 4, 5];
        let factorials = [
            2,
            2 * 3,
            2 * 3 * 4,
            2 * 3 * 4 * 5
        ];
        for (input, expected_output) in inputs.into_iter().zip(factorials.into_iter()) {
            assert_eq!(factorial(input), Some(expected_output));
        }
    }

    #[test]
    fn overflow_test() {
        assert_eq!(factorial(99), None)
    }
}