const MAX_TMP_RESULT: i32 = ('@' as i32 - '.' as i32) * 4;
pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split("\n").collect();
    let mut result = 0;
    let mut map: Vec<Vec<i8>> = vec![];
    for line in puzzle_input.iter() {
        let vec_of_chars = line.chars().collect::<Vec<char>>();
        let mut vec_of_numbers: Vec<i8> = vec![];
        for c in vec_of_chars.iter() {
            vec_of_numbers.push(*c as i8 - '.' as i8);
        }
        map.push(vec_of_numbers);
    }

    loop {
        let tmp_res = result.clone();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                let is_paper = map[i][j] == 18;
                if !is_paper {
                    continue;
                }
                if i == 0 {
                    if j == 0 || j == map[i].len() - 1 {
                        // top corners
                        result += 1;
                        map[i][j] = 0;
                        continue;
                    }
                    let tmp_result = map[i][j - 1] as i32
                        + map[i][j + 1] as i32
                        + map[i + 1][j] as i32
                        + map[i + 1][j - 1] as i32
                        + map[i + 1][j + 1] as i32;
                    if tmp_result < MAX_TMP_RESULT {
                        result += 1;
                        map[i][j] = 0;
                    }
                } else if i == map.len() - 1 {
                    if j == 0 || j == map[i].len() - 1 {
                        // bottom corners
                        result += 1;
                        map[i][j] = 0;
                        continue;
                    }
                    let tmp_result = map[i - 1][j] as i32
                        + map[i - 1][j - 1] as i32
                        + map[i - 1][j + 1] as i32
                        + map[i][j - 1] as i32
                        + map[i][j + 1] as i32;
                    if tmp_result < MAX_TMP_RESULT {
                        result += 1;
                        map[i][j] = 0;
                    }
                } else {
                    if j == 0 {
                        // can check all 8 sides around
                        let tmp_result = map[i - 1][j] as i32
                            + map[i - 1][j + 1] as i32
                            + map[i][j + 1] as i32
                            + map[i + 1][j] as i32
                            + map[i + 1][j + 1] as i32;
                        if tmp_result < MAX_TMP_RESULT {
                            result += 1;
                            map[i][j] = 0;
                        }
                    } else if j == map[i].len() - 1 {
                        // can check all 8 sides around
                        let tmp_result = map[i - 1][j] as i32
                            + map[i - 1][j - 1] as i32
                            + map[i][j - 1] as i32
                            + map[i + 1][j] as i32
                            + map[i + 1][j - 1] as i32;
                        if tmp_result < MAX_TMP_RESULT {
                            result += 1;
                            map[i][j] = 0;
                        }
                    } else {
                        // can check all 8 sides around
                        let tmp_result = map[i - 1][j] as i32
                            + map[i - 1][j - 1] as i32
                            + map[i - 1][j + 1] as i32
                            + map[i][j - 1] as i32
                            + map[i][j + 1] as i32
                            + map[i + 1][j] as i32
                            + map[i + 1][j - 1] as i32
                            + map[i + 1][j + 1] as i32;
                        if tmp_result < MAX_TMP_RESULT {
                            result += 1;
                            map[i][j] = 0;
                        }
                    }
                }
            }
        }
        if tmp_res == result {
            break;
        }
    }

    println!("Day 3 Part 2 result is: {result}");
}
