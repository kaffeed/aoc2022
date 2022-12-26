use super::*;

pub fn run(file_name: &str) {
    let (mut inputs, instructions) = read_input(file_name);

    parse_instructions(&instructions)
        .into_iter()
        .for_each(|(count, source, target)| {
            for _ in 0..count {
                if let Some(val) = inputs[source].pop() {
                    inputs[target].push(val);
                }
            }

        });

    let res = create_output(&inputs);

    println!("Output for day five part one: {res}");
}

