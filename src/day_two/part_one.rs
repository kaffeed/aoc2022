use super::*;

fn convert_to_move(m: &str) -> Move {
    match m {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissor,
        _ => panic!("No other character should exist"),
    }
}

fn convert_line_to_move(line: String) -> (Move, Move) {
    let move_vec = line
        .trim()
        .split_whitespace()
        .map(|m| convert_to_move(m))
        .collect::<Vec<Move>>();

    (move_vec[0], move_vec[1])
}

pub fn run(input: &str) {
    println!(
        "Output day_two part one:\t{}",
        run_internal(read_file(input, convert_line_to_move))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_points() {
        assert_eq!(calculate_points(convert_line_to_move("A Y".to_string())), 8);
        assert_eq!(calculate_points(convert_line_to_move("B X".to_string())), 1);
        assert_eq!(calculate_points(convert_line_to_move("C Z".to_string())), 6);
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

        assert_eq!(run_internal(Ok(v)), 15);
    }

    #[test]
    #[should_panic(expected = "No other character should exist")]
    fn test_convert_to_move_panic() {
        convert_to_move("af");
        convert_to_move("a");
    }

    #[test]
    fn test_convert_line_to_move() {
        assert_eq!(
            convert_line_to_move("A Y".to_string()),
            (Move::Rock, Move::Paper)
        );
        assert_eq!(
            convert_line_to_move("B X".to_string()),
            (Move::Paper, Move::Rock)
        );
        assert_eq!(
            convert_line_to_move("C Z".to_string()),
            (Move::Scissor, Move::Scissor)
        );
    }
}
