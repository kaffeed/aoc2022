use std::fs;

fn read_file_to_string(input: &str) -> String {
    fs::read_to_string(input).expect("The input file should exist!")
}

fn transform_input(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|splits| {
            splits
                .split_whitespace()
                .map(|cal_as_string| cal_as_string.trim())
                .map(|cal_as_string| cal_as_string.parse::<i32>().expect("should be an integer"))
                .collect::<Vec<i32>>()
                .iter()
                .sum()
        })
        .collect::<Vec<i32>>()
}

fn get_maximum_calories(input: Vec<i32>) -> Option<i32> {
    input.iter().max().copied()
}

pub mod part_one {
    use super::*;

    pub fn run(input: &str) {
        let input = transform_input(&read_file_to_string(input));
        println!("Output day_one part one:\t{}", get_maximum_calories(input).unwrap());
    }
}

pub mod part_two {
    use super::*;

    pub fn run(input: &str) {
        let mut input = transform_input(&read_file_to_string(input));
        input.sort();
        input.reverse();
        println!("Output day_one part two:\t{}", (&input[..3]).iter().sum::<i32>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "The input file should exist!")]
    fn test_read_input_does_not_exist() {
        read_file_to_string("filename that does not exist");
    }

    #[test]
    fn test_read_input() {
        let test_input =
            transform_input("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(test_input.len(), 5);
        assert_eq!(*test_input.get(0).unwrap(), 6000 as i32);
        assert_eq!(*test_input.get(1).unwrap(), 4000 as i32);
        assert_eq!(*test_input.get(2).unwrap(), 11000 as i32);
        assert_eq!(*test_input.get(3).unwrap(), 24000 as i32);
        assert_eq!(*test_input.get(4).unwrap(), 10000 as i32);
    }

    #[test]
    fn test_get_max_value() {
        let test_input =
            transform_input("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(get_maximum_calories(test_input), Some(24000));
    }
}
