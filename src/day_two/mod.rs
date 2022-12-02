use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod part_one;
pub mod part_two;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

const WIN_POINTS: i32 = 6;
const DRAW_POINTS: i32 = 3;

fn calculate_points(duel: (Move, Move)) -> i32 {
    match duel.0 {
        Move::Rock => match duel.1 {
            Move::Rock => DRAW_POINTS + 1,
            Move::Paper => WIN_POINTS + 2,
            Move::Scissor => 3,
        },
        Move::Paper => match duel.1 {
            Move::Rock => 1,
            Move::Paper => DRAW_POINTS + 2,
            Move::Scissor => WIN_POINTS + 3,
        },
        Move::Scissor => match duel.1 {
            Move::Rock => WIN_POINTS + 1,
            Move::Paper => 2,
            Move::Scissor => DRAW_POINTS + 3,
        },
    }
}

fn run_internal(input: std::io::Result<Vec<(Move, Move)>>) -> i32 {
    input
        .expect("file should exist")
        .into_iter()
        .map(|m| calculate_points(m))
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn read_file<P>(
    file_name: P,
    converter: impl Fn(String) -> (Move, Move),
) -> std::io::Result<Vec<(Move, Move)>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name).expect("File should exist");
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| converter(line.expect("line should exist")))
        .collect())
}

