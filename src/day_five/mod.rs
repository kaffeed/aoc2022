use std::fs::read_to_string;
pub mod part_one;
pub mod part_two;


fn read_input(file_name: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let lines = read_to_string(file_name)
        .expect("file should exist")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let empty_pos = lines
        .iter()
        .position(|line| line.is_empty())
        .expect("should exist");

    let mut stack = lines[..empty_pos].to_vec();
    stack.reverse();

    let mut stackiter = stack.into_iter();

    let number_of_stacks = stackiter
        .next()
        .expect("should exist")
        .split(' ')
        .filter(|num| !num.is_empty())
        .count();

    let mut stacks: Vec<Vec<String>> = vec![];

    for _ in 0..number_of_stacks {
        stacks.push(vec![]);
    }

    stackiter.for_each(|line| {
        let mut empty_count = 0;
        let mut is_last_empty = true;

        line.split(' ').enumerate().for_each(|(i, c)| {
            if !c.is_empty() {
                stacks[i - empty_count + (empty_count / 4)].push(c.to_string());
                is_last_empty = false;
            } else {
                is_last_empty = true;
                empty_count += 1;
            }
        });
    });

    (stacks, lines[empty_pos + 1..].to_vec())
}

fn parse_instructions(instructions: &Vec<String>) -> Vec<(usize, usize, usize)> {
    instructions.into_iter().map(|line| {
        let mut split = line.split(' ').filter(|x| !x.is_empty());
        split.next(); // move
        let count = split
            .next()
            .expect("Should be some")
            .parse::<usize>()
            .expect("should be a number");

        split.next(); // from
        let source = split
            .next()
            .expect("Should be some")
            .parse::<usize>()
            .expect("should be a number");

        split.next(); // to
        let target = split
            .next()
            .expect("Should be some")
            .parse::<usize>()
            .expect("should be a number");

        (count, source - 1, target - 1)
    }).collect()
}

fn create_output(input: &Vec<Vec<String>>) -> String {
    input.into_iter().fold("".to_string(), |acc, s| {
        if let Some(last) = s.last() {
            let s = last.trim_start_matches("[").trim_end_matches("]");
            acc + s
        } else {
            acc
        }
    })
}

