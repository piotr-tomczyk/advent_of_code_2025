pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split("\n").collect();
    let mut fresh_ids_ranges: Vec<(u128, u128)> = vec![];
    let mut result: u128 = 0;
    for line in puzzle_input.iter() {
        if *line == "" {
            break;
        }
        let id_range_split: Vec<&str> = line.split('-').collect();
        let left_boundary: u128 = id_range_split[0].parse::<u128>().unwrap();
        let right_boundary: u128 = id_range_split[1].parse::<u128>().unwrap();
        fresh_ids_ranges.push((left_boundary, right_boundary));
        fresh_ids_ranges.sort();
        let mut did_remove_happen = true;
        while did_remove_happen == true {
            did_remove_happen = false;
            for index_to_check in 0..fresh_ids_ranges.len() {
                let range_to_check = fresh_ids_ranges[index_to_check];
                for index in 0..fresh_ids_ranges.len() {
                    if index_to_check == index {
                        continue;
                    }
                    let range = fresh_ids_ranges[index];
                    let is_left_boundary_in_range =
                        range_to_check.0 >= range.0 && range_to_check.0 <= range.1;
                    let is_right_boundary_in_range =
                        range_to_check.1 >= range.0 && range_to_check.1 <= range.1;
                    if is_left_boundary_in_range && is_right_boundary_in_range {
                        fresh_ids_ranges.remove(index_to_check);
                        fresh_ids_ranges.sort();
                        did_remove_happen = true;
                        break;
                    } else if !is_left_boundary_in_range && is_right_boundary_in_range {
                        fresh_ids_ranges[index] = (range_to_check.0, range.1);
                        fresh_ids_ranges.remove(index_to_check);
                        fresh_ids_ranges.sort();
                        did_remove_happen = true;
                        break;
                    } else if is_left_boundary_in_range && !is_left_boundary_in_range {
                        fresh_ids_ranges[index] = (range.0, range_to_check.1);
                        fresh_ids_ranges.remove(index_to_check);
                        fresh_ids_ranges.sort();
                        did_remove_happen = true;
                        break;
                    }
                }
                if did_remove_happen == true {
                    break;
                }
            }
        }
    }
    fresh_ids_ranges.sort();
    for range in fresh_ids_ranges.iter() {
        result += range.1 + 1 - range.0;
    }

    println!("Day 5 Part 1 result is: {result}");
}
