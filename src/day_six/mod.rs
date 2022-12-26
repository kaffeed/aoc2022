use std::{collections::BTreeSet, fs};

pub mod part_one;
pub mod part_two;

fn run_internal(file_name: &str, window_size: usize) -> Option<i32> {
    for (i, win) in read_input(file_name)
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
    {
        let mut set = BTreeSet::new();
        win.iter().for_each(|c| {
            set.insert(*c);
        });

        if set.len() == window_size {
            return Some((i + window_size) as i32);
        }
    }
    return None;
}

fn read_input(file_name: &str) -> String {
    fs::read_to_string(file_name)
        .expect("file should exist")
        .trim()
        .to_string()
}
