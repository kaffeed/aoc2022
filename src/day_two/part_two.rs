use super::*;

fn convert_line_to_move(line: String) -> (Move, Move) {
    let move_vec = line
        .trim()
        .split_whitespace()
        // .map(|m| convert_to_move(m))
        .collect::<Vec<&str>>();

    (
        convert_opponents_move(move_vec[0]),
        convert_own_move(move_vec[1], convert_opponents_move(move_vec[0])),
    )
}

pub fn run(input: &str) {
    println!(
        "Output day_two part two:\t{}",
        run_internal(read_file(input, convert_line_to_move))
    );
}

fn convert_own_move(m: &str, opponents_move: Move) -> Move {
    match m {
        "X" => match opponents_move {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        },
        "Y" => match opponents_move {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissor => Move::Scissor,
        },
        "Z" => match opponents_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        },
        _ => panic!("No other character should exist"),
    }
}

fn convert_opponents_move(m: &str) -> Move {
    match m {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissor,
        _ => panic!("No other character should exist"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_points() {
        assert_eq!(calculate_points(convert_line_to_move("A Y".to_string())), 4);
        assert_eq!(calculate_points(convert_line_to_move("B X".to_string())), 1);
        assert_eq!(calculate_points(convert_line_to_move("C Z".to_string())), 7);
    }

    #[test]
    #[should_panic(expected = "File should exist")]
    fn test_read_file_should_return_error() {
        let _ = read_file("Non existent", convert_line_to_move);
    }

    #[test]
    fn test_sumup() {
        let v = vec![
            convert_line_to_move("A Y".to_string()),
            convert_line_to_move("B X".to_string()),
            convert_line_to_move("C Z".to_string()),
        ];

        assert_eq!(run_internal(Ok(v)), 12);
    }

    #[test]
    #[should_panic(expected = "No other character should exist")]
    fn test_convert_to_move_panic() {
        convert_opponents_move("a");
    }

    #[test]
    fn test_convert_line_to_move() {
        assert_eq!(
            convert_line_to_move("A Y".to_string()),
            (Move::Rock, Move::Rock)
        );
        assert_eq!(
            convert_line_to_move("B X".to_string()),
            (Move::Paper, Move::Rock)
        );
        assert_eq!(
            convert_line_to_move("C Z".to_string()),
            (Move::Scissor, Move::Rock)
        );
    }
}
