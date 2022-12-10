use super::*;

fn calculate_sum_of_priorities(input: Vec<String>) -> i32 {
    input
        .chunks(3)
        .map(|c| intersect_multiple(c.to_vec()))
        .map(|c| match c {
            Some(val) => calculate_priority(val[0]) as i32,
            None => panic!("Should not happen!"),
        })
        .sum()
}

pub fn run(input: &str) {
    let input = read_input(input);
    println!(
        "Output day_three part three:\t{}",
        calculate_sum_of_priorities(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_priorities() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(70, calculate_sum_of_priorities(input));
    }
}

