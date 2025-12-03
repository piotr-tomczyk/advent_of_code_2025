pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split("\n").collect();
    let mut result = 0;
    for batteries in puzzle_input.iter() {
        let mut the_highest = (0, 0);
        let mut the_second_highest = (0, 0);
        for (index, battery) in batteries.chars().enumerate() {
            let battery_as_int = battery as i32 - '0' as i32;

            match battery_as_int {
                b if b > the_highest.1 => {
                    if index + 1 != batteries.len() {
                        the_second_highest = (0, 0);
                    } else {
                        the_second_highest = the_highest.clone();
                    }
                    the_highest = (index, battery_as_int);
                }
                b if b > the_second_highest.1 => {
                    the_second_highest = (index, battery_as_int);
                }
                _ => {}
            }
        }
        if the_highest.0 < the_second_highest.0 {
            result += (the_highest.1 * 10) + the_second_highest.1;
        } else {
            result += (the_second_highest.1 * 10) + the_highest.1;
        }
    }

    println!("Day 3 Part 1 result is: {result}");
}
