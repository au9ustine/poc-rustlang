use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum OperationsError {
    DividedByZeroError,
}

impl fmt::Display for OperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OperationsError::DividedByZeroError => {
                f.write_str("Cannot divided by zero")
            }
        }
    }
}

impl Error for OperationsError {
    fn description(&self) -> &str {
        match *self {
            OperationsError::DividedByZeroError => "Cannot divided by zero",
        }
    }
}

fn divide(dividend: u32, divisor: u32) -> Result<u32, OperationsError> {
    if divisor == 0u32 {
        Err(OperationsError::DividedByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let result1 = divide(100, 0);
    println!("{:?}", result1);
    let result2 = divide(100, 2);
    println!("{:?}", result2.unwrap());
}
