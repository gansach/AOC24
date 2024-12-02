mod utils;
mod day01;

fn main() {
    let day = 1;
    let part = 'B';

    let _filename = format!("src/inputs/day{:02}.txt", day);
    let _filename_with_part = format!("src/inputs/day{:02}_{}.txt", day, part);

    match (day, part) {
        (1, 'A') => day01::run_part_a(&_filename),
        (1, 'B') => day01::run_part_b(&_filename),
        _ => println!("Day or part not implemented!"),
    }
}
