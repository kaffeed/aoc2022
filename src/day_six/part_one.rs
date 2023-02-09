use super::*;

pub fn run(file_name: &str) {
    println!(
        "Output for day six part one: {}",
        run_internal(file_name, 4).unwrap()
    );
}
