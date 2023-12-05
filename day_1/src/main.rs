fn main() {
    const RADIX: u32 = 10;
    let string_lines = include_str!("../input.txt")
                                    .lines() ;
    let mut all_values: Vec<i32> = Vec::new();
    let valid_nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for mut line in string_lines {

        while !valid_nums.iter().any(|x| line.starts_with(x))
            && !line.chars().next().unwrap().is_numeric()
        {   
            line = &line[1..];
        }
        while !valid_nums.iter().any(|x| line.ends_with(x)) && !line.chars().last().unwrap().is_numeric()
        {   

            line = &line[..line.len() - 1];
        }
        let mut left_index = 0;
        let mut right_index = 0;
        for (x, num) in valid_nums.iter().enumerate() {
            if line.starts_with(num) {
                left_index = x + 1;
            }
            if line.ends_with(num) {
                right_index = x + 1;
            }
        }
        if line.chars().next().unwrap().is_numeric() {
            left_index = line.chars().next().unwrap().to_digit(RADIX).unwrap() as usize;
        }
        if line.chars().last().unwrap().is_numeric() {
            right_index = line.chars().last().unwrap().to_digit(RADIX).unwrap() as usize;
        }
        // println!("{:?}", left_index);
        // println!("{:?}", right_index);
        all_values.push((left_index * 10 + right_index).try_into().unwrap());
        // let left_index: usize= line.find(char::is_numeric).unwrap() ;
        // let right_index: usize= line.rfind(char::is_numeric).unwrap();
        // let current_number: i32 = format!("{}{}",line.chars().nth(left_index).unwrap(), line.chars().nth(right_index).unwrap()).parse().unwrap();
        // all_values.push(current_number);

    }
    let sum: i32 = all_values.iter().sum();
    println!("the total sum is: {}", sum);
}
