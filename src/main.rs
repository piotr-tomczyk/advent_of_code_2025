mod demo;

const DAY: u8 = 1;
const PART: u8 = 2;

fn main() {
    match (DAY, PART) {
        (1, 1) => demo::solution_1::run(),
        (1, 2) => demo::solution_2::run(),
        _ => println!("Day {} Part {} not implemented", DAY, PART),
    }
}
