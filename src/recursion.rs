// recursion.rs
// a program that uses recursion to find a key in nested boxes.

pub fn main() {
    println!("=== Recursion ===");

    println!("Factorial Expected: {} Actual: {}", 120, factorial(5));
    println!("Factorial Expected: {} Actual: {}", 6, factorial(3));
    println!("Factorial Expected: {} Actual: {}", 5040, factorial(7));
}

/// Calculates the factorial.
/// A factorial is the product of all whole numbers for a given number down to 1.
/// Formular: n! = n * !(n - 1)
///
/// Return the factorial number.
fn factorial(num: i64) -> i64 {
    if num == 1 {
        // Base case
        return num;
    } else {
        // Recursive case
        num * factorial(num - 1)
    }
}
