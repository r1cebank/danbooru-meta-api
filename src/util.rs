extern crate rand;

use rand::Rng;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorType {
    pub details: String,
}

impl ErrorType {
    fn new(msg: &str) -> ErrorType {
        ErrorType {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ErrorType {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn get_rand_ids(start: i32, end: i32, size: i32) -> Result<Vec<i32>, ErrorType> {
    if size > (end - start) {
        Err(ErrorType::new("Range out of bounds"))
    } else {
        let mut rng = rand::thread_rng();
        let mut numbers = Vec::new();
        while (numbers.len() as i32) < size {
            let random_number: i32 = rng.gen_range(start, end);
            if !numbers.contains(&random_number) {
                numbers.push(random_number);
            }
        }
        Ok(numbers)
    }
}
