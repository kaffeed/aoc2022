use super::*;

pub fn run(input: &str) {
    let mut input = transform_input(&read_file_to_string(input));
    input.sort();
    input.reverse();
    println!("Output day_one part two:\t{}", (&input[..3]).iter().sum::<i32>());
}

