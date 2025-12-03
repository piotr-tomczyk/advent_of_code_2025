const ARRAY_SIZE: usize = 12;
pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split("\n").collect();
    let mut result: u128 = 0;
    for batteries in puzzle_input.iter() {
        let mut highest_voltage: [(usize, u8); ARRAY_SIZE] = [(0, 0); ARRAY_SIZE];
        for i in 0..ARRAY_SIZE {
            for (index, battery) in batteries.chars().enumerate() {
                if i > 0 {
                    if index <= highest_voltage[i - 1].0 {
                        continue;
                    }
                }
                if (batteries.len() - ARRAY_SIZE + i) < index {
                    break;
                }
                let battery_as_u8 = battery as u8 - '0' as u8;
                if battery_as_u8 > highest_voltage[i].1 {
                    highest_voltage[i] = (index, battery_as_u8);
                }
            }
            result += u128::pow(10, (ARRAY_SIZE - 1 - i) as u32) * highest_voltage[i].1 as u128;
        }
    }

    println!("Day 3 Part 2 result is: {result}");
}
