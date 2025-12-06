pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split("\n").collect();
    let mut result: u128 = 0;
    let mut operators: Vec<u8> = vec![];
    let operators_line = puzzle_input[puzzle_input.len() - 1];
    for operator in operators_line.chars() {
        match operator {
            '*' => operators.push(0),
            '+' => operators.push(1),
            _ => {}
        }
    }

    let mut equations: Vec<Vec<(u8, u128)>> = vec![];
    let mut line_0: Vec<&str> = puzzle_input[0].split(' ').collect();
    line_0.retain(|&x| x != "");

    for _i in 0..line_0.len() {
        equations.push(vec![]);
    }
    for line in puzzle_input.iter() {
        if line.contains('*') {
            break;
        }
        let mut line_split_to_nums: Vec<&str> = line.split(' ').collect();
        line_split_to_nums.retain(|&x| x != "");
        for (index, number_as_str) in line_split_to_nums.iter().enumerate() {
            if *number_as_str == "" {
                continue;
            }
            let number = number_as_str.parse::<u128>().unwrap();
            equations[index].push((operators[index], number));
        }
    }

    for equation in equations.iter() {
        let mut tmp_result: u128 = 0;
        for (operation, number) in equation.iter() {
            if tmp_result == 0 {
                tmp_result = *number;
                continue;
            }
            match *operation {
                0 => {
                    tmp_result *= *number;
                }
                1 => {
                    tmp_result += *number;
                }
                _ => {}
            }
        }
        result += tmp_result;
    }

    println!("Day 6 Part 1 result is: {result}");
}
