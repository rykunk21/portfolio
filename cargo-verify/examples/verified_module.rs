//! Example: Using cargo-verify Datalog declarations
//!
//! This file demonstrates the docstring format that cargo-verify expects.

/// Adds two numbers
///
/// ```datalog
/// test("add", pass).
/// coverage("add", 100.0).
/// complexity("add", 1).
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divides two numbers
///
/// ```datalog
/// test("div", pass).
/// test("div_by_zero", fail).
/// coverage("div", 95.0).
/// complexity("div", 3).
/// ```
pub fn div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Complex processing function
///
/// ```dolog
/// % This is a comment - ignored by parser
/// test("process_data", pass).
/// complexity("process_data", 15).
/// ```
pub fn process_data(input: &str) -> String {
    input.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_div() {
        assert_eq!(div(10.0, 2.0), Some(5.0));
    }
    
    #[test]
    fn test_div_by_zero() {
        assert_eq!(div(10.0, 0.0), None);
    }
    
    #[test]
    fn test_process_data() {
        assert_eq!(process_data("hello"), "HELLO");
    }
}