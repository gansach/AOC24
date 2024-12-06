mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let day = 6;
    let part = 'B';

    let _filename = format!("src/inputs/day{:02}.txt", day);
    let _filename_with_part = format!("src/inputs/day{:02}_{}.txt", day, part);

    match (day, part) {
        (1, 'A') => day01::run_part_a(&_filename),
        (1, 'B') => day01::run_part_b(&_filename),
        (2, 'A') => day02::run_part_a(&_filename),
        (2, 'B') => day02::run_part_b(&_filename),
        (3, 'A') => day03::run_part_a(&_filename),
        (3, 'B') => day03::run_part_b(&_filename),
        (4, 'A') => day04::run_part_a(&_filename),
        (4, 'B') => day04::run_part_b(&_filename),
        (5, 'A') => day05::run_part_a(&_filename),
        (5, 'B') => day05::run_part_b(&_filename),
        (6, 'A') => day06::run_part_a(&_filename),
        (6, 'B') => day06::run_part_b(&_filename),
        _ => println!("Day or part not implemented!"),
    }
}
