pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split("\n").collect();
    let mut fresh_ids_ranges: Vec<(u128, u128)> = vec![];
    let mut result: u16 = 0;
    let mut break_found = false;
    for line in puzzle_input.iter() {
        if *line == "" {
            break_found = true;
            continue;
        }
        if break_found {
            for fresh_id_range in fresh_ids_ranges.iter() {
                let id_as_u = line.parse::<u128>().unwrap();
                if id_as_u >= fresh_id_range.0 && id_as_u <= fresh_id_range.1 {
                    result += 1;
                    break;
                }
            }
        } else {
            let id_range_split: Vec<&str> = line.split('-').collect();
            let left_boundary: u128 = id_range_split[0].parse::<u128>().unwrap();
            let right_boundary: u128 = id_range_split[1].parse::<u128>().unwrap();
            fresh_ids_ranges.push((left_boundary, right_boundary));
        }
    }

    println!("Day 5 Part 1 result is: {result}");
}
