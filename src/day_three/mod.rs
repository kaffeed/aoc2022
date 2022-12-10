use std::{collections::BTreeSet, fs};

pub mod part_one;
pub mod part_two;


fn calculate_priority(letter: char) -> u8 {
    match letter.is_lowercase() {
        true => (letter as u8) - b'a' + 1,
        false => (letter as u8) - b'A' + 26 + 1,
    }
}

fn read_input(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect("File should exist")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn split_at_half(input: &str) -> (&str, &str) {
    let middle = input.len() / 2;
    (&input[..middle], &input[middle..])
}

pub fn intersect_multiple(input: Vec<String>) -> Option<Vec<char>> {
    let trees = input
        .iter()
        .map(|s| BTreeSet::from_iter(s.chars()))
        .collect::<Vec<BTreeSet<char>>>();

    let first = trees[0].clone();

    let common_chars = trees[1..]
        .iter()
        .fold(first, |acc, s| acc.intersection(&s).cloned().collect());

    if common_chars.is_empty() {
        None
    } else {
        Some(common_chars.into_iter().collect::<Vec<char>>())
    }
}

fn intersect(first: &str, second: &str) -> char {
    // let chars = input.0.chars();
    let first_input: BTreeSet<char> = BTreeSet::from_iter(first.chars());
    let second_input: BTreeSet<char> = BTreeSet::from_iter(second.chars());

    let mut intersection = first_input.intersection(&second_input);
    intersection
        .next()
        .expect("there should be an intersection")
        .clone()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "File should exist")]
    fn test_panic_on_file_does_not_exist() {
        read_input("asdf");
    }

    #[test]
    fn test_split_in_half() {
        let res = split_at_half("asdfzxcv");

        assert_eq!(res.0, "asdf");
        assert_eq!(res.1, "zxcv");
    }

    #[test]
    fn test_calculate_priority() {
        assert_eq!(1, calculate_priority('a'));
        assert_eq!(26, calculate_priority('z'));
        assert_eq!(27, calculate_priority('A'));
        assert_eq!(52, calculate_priority('Z'));
        assert_eq!(16, calculate_priority('p'));
        assert_eq!(38, calculate_priority('L'));
        assert_eq!(42, calculate_priority('P'));
        assert_eq!(22, calculate_priority('v'));
        assert_eq!(20, calculate_priority('t'));
        assert_eq!(19, calculate_priority('s'));
    }

    #[test]
    #[should_panic(expected = "there should be an intersection")]
    fn test_no_element_in_common_in_intersection() {
        intersect("asdf", "zxcv");
    }

    #[test]
    fn test_intersection_of_strings() {
        assert_eq!('p', intersect("aaspdf", "poiuj"));
        assert_eq!('a', intersect("aa", "a"));
    }

    #[test]
    fn test_intersect_multiple() {
        assert_eq!(Some(vec!['p']), intersect_multiple(vec!["aaspdf".to_string(), "poiuj".to_string()]));
    }
}
