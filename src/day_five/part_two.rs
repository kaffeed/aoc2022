use super::*;

pub fn run(file_name: &str) {
    let (mut inputs, instructions) = read_input(file_name);

    parse_instructions(&instructions)
        .into_iter()
        .for_each(|(count, source, target)| {
            let mut to_move: Vec<String> = vec![];
            for _ in 0..count {
                if let Some(val) = inputs[source].pop() {
                    to_move.push(val);
                }
            }

            to_move.reverse();
            to_move.into_iter().for_each(|val| {
                inputs[target].push(val);
            });
        });

    println!("Output for day five part one: {}", create_output(&inputs));
}

