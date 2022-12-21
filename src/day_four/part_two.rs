use super::*;

fn overlap(p1: (i32, i32), p2: (i32, i32)) -> bool {
    (p1.1 >= p2.0 && p2.0 >= p1.0 || p2.0 >= p1.0 && p2.0 <= p1.1)
        || (p1.0 >= p2.0 && p1.0 <= p2.1 || p2.1 >= p1.0 && p2.1 <= p1.1)
}

pub fn run(file_name: &str) {
    let count = read_input(file_name, overlap)
        .into_iter()
        .filter(|x| *x)
        .count();
    println!("Output day_four part two:\t{}", count);
}

