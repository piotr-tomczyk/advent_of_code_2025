pub fn run() {
    let x = include_str!("./example.txt").split("\n");
    let y: Vec<&str> = x.collect();
    let mut numbers_left: Vec<i32> = vec![];
    let mut numbers_right: Vec<i32> = vec![];
    let mut result = 0;
    for line in y.iter() {
        let nums: Vec<&str> = line.split("   ").collect();
        numbers_left.push(nums[0].parse().unwrap());
        numbers_right.push(nums[1].parse().unwrap());
    }
    numbers_left.sort();
    numbers_right.sort();
    for (line_left, line_right) in numbers_left.iter().zip(numbers_right.iter()) {
        result += (line_left - line_right).abs();
    }

    println!("Result first is: {}", result);
}
