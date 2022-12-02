use super::*;

pub fn run(input: &str) {
    let input = transform_input(&read_file_to_string(input));
    println!("Output day_one part one:\t{}", get_maximum_calories(input).unwrap());
}

