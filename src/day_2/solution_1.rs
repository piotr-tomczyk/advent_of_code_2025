pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split(",").collect();
    let mut result: u128 = 0;
    for id_range in puzzle_input.iter() {
        let id_range_split: Vec<&str> = id_range.split("-").collect();
        let id_left: u128 = id_range_split[0].parse::<u128>().unwrap();
        let id_right: u128 = id_range_split[1].parse::<u128>().unwrap();

        for id in id_left..=id_right {
            let id_stringified = id.to_string();
            let string_len = id_stringified.len();
            if string_len % 2 != 0 {
                continue;
            }
            let id_stringified_left = match id_stringified.get(0..string_len / 2) {
                Some(value) => value,
                None => "",
            };
            let id_stringified_right = match id_stringified.get(string_len / 2..string_len) {
                Some(value) => value,
                None => "",
            };
            if id_stringified_left == id_stringified_right {
                result += id;
            }
        }
    }

    println!("Day 2 Part 1 result is: {result}");
}
