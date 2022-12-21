use std::{fs};

pub mod part_one;
pub mod part_two;

fn read_input(file_name: &str, comp_fun: fn((i32, i32), (i32, i32)) -> bool) -> Vec<bool> {
    fs::read_to_string(file_name)
        .expect("file should exist")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| line.split(','))
        .map(|mut parts| {
            let p1 = match parts.next() {
                Some(first_elf) => {
                    let mut split = first_elf.split('-');
                    let first: i32 = split.next().unwrap().parse().unwrap();
                    let second: i32 = split.next().unwrap().parse().unwrap();
                    (first, second)
                }
                None => panic!("Should not happen!"),
            };
            let p2 = match parts.next() {
                Some(first_elf) => {
                    let mut split = first_elf.split('-');
                    let first: i32 = split.next().unwrap().parse().unwrap();
                    let second: i32 = split.next().unwrap().parse().unwrap();
                    (first, second)
                }
                None => panic!("Should not happen!"),
            };

            comp_fun(p1, p2)
        })
        .collect()
}

