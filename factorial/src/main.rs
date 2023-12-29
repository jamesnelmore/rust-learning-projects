// Problem 1: Write a program which finds the factorial of a number entered by the user. (check for all conditions).

use std::io;
fn main() {
    // take user input
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {},
        Err(error) => { println!("{error}"); }
    }
    // TODO coerce into u64 or fail

    // print
}

// Naive implementation
fn factorial(x: u128) -> u128 {
    if x == 0 {
        return 1
    }
    
    let mut product = 1;
    for i in 1..=x {
        product *= i
    }

    product
}



#[cfg(test)]
mod factorial_tests {
    use super::*;

    #[test]
    fn factorial_0() { assert_eq!(factorial(0), 1); }

    #[test]
    fn factorial_1() { assert_eq!(factorial(1), 1);}
    
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
            assert_eq!(factorial(input), expected_output);
        }
    }
}