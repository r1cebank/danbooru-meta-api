extern crate rand;

use crate::models;
use rand::seq::SliceRandom;
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

pub fn get_rand_ids<R: Rng>(
    start: u32,
    end: u32,
    size: u32,
    rng: &mut R,
) -> Result<Vec<u32>, ErrorType> {
    if size > (end - start) {
        Err(ErrorType::new("Range out of bounds"))
    } else {
        let mut numbers = Vec::new();
        while (numbers.len() as u32) < size {
            let random_number: u32 = rng.gen_range(start, end);
            if !numbers.contains(&random_number) {
                numbers.push(random_number);
            }
        }
        Ok(numbers)
    }
}

pub fn create_batches<R: Rng>(
    total_posts: u32,
    batch_size: u32,
    validation_split: u8,
    test_split: u8,
    rng: &mut R,
) -> models::Batches {
    // 1. Calculate number of batches using total_posts / batch_size * 2
    let partition_size = batch_size * 2;
    let num_batches = total_posts / partition_size;
    let mut num_validation = (num_batches as f32 * validation_split as f32 / 100.0) as u32;
    let mut num_test = (num_batches as f32 * test_split as f32 / 100.0) as u32;
    if num_validation == 0 {
        num_validation = 1;
    }
    if num_test == 0 {
        num_test = 1;
    }
    let num_train = num_batches - (num_validation + num_test);
    let mut partitions = Vec::new();
    // 2. Calculate the bounds for the partition
    for batch_number in 0..num_batches {
        partitions.push((
            batch_number * partition_size + 1,
            (batch_number + 1) * partition_size,
        ));
    }
    let mut partition_ids = Vec::new();
    for partition in partitions.into_iter() {
        partition_ids.push(get_rand_ids(partition.0, partition.1 + 1, batch_size, rng).unwrap());
    }
    // Shuffle the ids so we are picking from random partitions
    partition_ids.shuffle(rng);
    let mut train_ids = Vec::new();
    let mut validation_ids = Vec::new();
    let mut test_ids = Vec::new();
    // 3. Using split percentage to randomly partition test and validation data
    for _ in 0..num_train {
        train_ids.push(partition_ids.pop().unwrap());
    }
    for _ in 0..num_validation {
        validation_ids.push(partition_ids.pop().unwrap());
    }
    for _ in 0..num_test {
        test_ids.push(partition_ids.pop().unwrap());
    }
    models::Batches {
        num_batches: num_batches,
        train: train_ids,
        validation: validation_ids,
        test: test_ids,
    }
}
