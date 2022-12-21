mod day_four;
mod day_one;
mod day_three;
mod day_two;

fn main() {
    day_one::part_one::run("inputs/day_one.txt");
    day_one::part_two::run("inputs/day_one.txt");
    day_two::part_one::run("inputs/day_two.txt");
    day_two::part_two::run("inputs/day_two.txt");
    day_three::part_one::run("inputs/day_three.txt");
    day_three::part_two::run("inputs/day_three.txt");
    day_four::part_one::run("inputs/day_four.txt");
    day_four::part_two::run("inputs/day_four.txt");
}
