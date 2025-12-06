mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod demo;

const DAY: u8 = 6;
const PART: u8 = 2;

fn main() {
    match (DAY, PART) {
        (0, 1) => demo::solution_1::run(),
        (0, 2) => demo::solution_2::run(),
        (1, 1) => day_1::solution_1::run(),
        (1, 2) => day_1::solution_2::run(),
        (2, 1) => day_2::solution_1::run(),
        (2, 2) => day_2::solution_2::run(),
        (3, 1) => day_3::solution_1::run(),
        (3, 2) => day_3::solution_2::run(),
        (4, 1) => day_4::solution_1::run(),
        (4, 2) => day_4::solution_2::run(),
        (5, 1) => day_5::solution_1::run(),
        (5, 2) => day_5::solution_2::run(),
        (6, 1) => day_6::solution_1::run(),
        (6, 2) => day_6::solution_2::run(),
        _ => println!("Day {} Part {} not implemented", DAY, PART),
    }
}
