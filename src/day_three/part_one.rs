use super::*;

fn calculate_sum_of_priorities(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| split_at_half(line))
        .map(|s| intersect(s.0, s.1))
        .map(|c| calculate_priority(c) as i32)
        .sum()
}

pub fn run(input: &str) {
    let input = read_input(input);
    println!(
        "Output day_three part one:\t{}",
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
        assert_eq!(157, calculate_sum_of_priorities(input));
    }
}

