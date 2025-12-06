pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split("\n").collect();
    let mut result: u128 = 0;
    let mut operators: Vec<u8> = vec![];
    let operators_line = puzzle_input[puzzle_input.len() - 1];
    let operators_line_as_chars = operators_line.chars().collect::<Vec<char>>();
    for operator in operators_line_as_chars.iter() {
        match operator {
            '*' => operators.push(0),
            '+' => operators.push(1),
            _ => {}
        }
    }

    let mut equations: Vec<Vec<(u8, String)>> = vec![];
    let mut line_0: Vec<&str> = puzzle_input[0].split(' ').collect();
    line_0.retain(|&x| x != "");
    for _i in 0..line_0.len() {
        equations.push(vec![]);
    }

    for line in puzzle_input.iter() {
        if line.contains('*') {
            break;
        }

        let mut index_of_eq = 0;
        let mut index_of_a_num = 0;
        for (index, eq_character) in line.chars().enumerate() {
            if eq_character == ' ' && index < operators_line.len() - 1 {
                if operators_line_as_chars[index + 1] == '*'
                    || operators_line_as_chars[index + 1] == '+'
                {
                    index_of_eq += 1;
                    index_of_a_num = 0;
                    continue;
                }
            }
            let eq_len = equations[index_of_eq].len() as i32;
            if eq_len - 1 < index_of_a_num {
                equations[index_of_eq].push((operators[index_of_eq], eq_character.to_string()));
            } else {
                equations[index_of_eq][index_of_a_num as usize].1 += &eq_character.to_string();
            }
            index_of_a_num += 1;
        }
    }

    for equation in equations.iter() {
        let mut tmp_result: u128 = 0;
        for (operation, number_as_string) in equation.iter() {
            let real_number = number_as_string.trim().parse::<u128>().unwrap();
            if tmp_result == 0 {
                tmp_result = real_number;
                continue;
            }
            match *operation {
                0 => {
                    tmp_result *= real_number;
                }
                1 => {
                    tmp_result += real_number;
                }
                _ => {}
            }
        }
        result += tmp_result;
    }

    println!("Day 6 Part 2 result is: {result}");
}
