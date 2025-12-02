pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split(",").collect();
    let mut result: u128 = 0;
    for id_range in puzzle_input.iter() {
        let id_range_split: Vec<&str> = id_range.split("-").collect();
        let id_left: u128 = id_range_split[0].parse::<u128>().unwrap();
        let id_right: u128 = id_range_split[1].parse::<u128>().unwrap();

        for id in id_left..=id_right {
            let id_stringified = id.to_string();

            for index in 0..id_stringified.len() {
                let digits_until_the_digit = match id_stringified.get(..=index) {
                    Some(value) => value,
                    None => "",
                };
                if id_stringified.len() % digits_until_the_digit.len() != 0 {
                    continue;
                }

                let id_stringified_split = id_stringified
                    .as_bytes()
                    .chunks(digits_until_the_digit.len())
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();
                if id_stringified_split.len() == 1 {
                    break;
                }

                let mut found = true;
                for string in id_stringified_split.iter() {
                    let x = &digits_until_the_digit;
                    if string != x {
                        found = false;
                        break;
                    }
                }
                if found == true {
                    result += id;
                    break;
                }
            }
        }
    }

    println!("Day 2 Part 2 result is: {result}");
}
