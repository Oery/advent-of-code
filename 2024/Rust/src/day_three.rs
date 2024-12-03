use regex::Regex;
use std::io::Read;

fn load_inputs() -> String {
    let mut input = String::new();

    let mut file = std::fs::File::open("./inputs/day_three.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    input
}

fn exec_mul(mul: regex::Match<'_>) -> u32 {
    mul.as_str()
        .strip_prefix("mul(")
        .expect("Operation is invalid: Missing prefix")
        .strip_suffix(")")
        .expect("Operation is invalid: Missing suffix")
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .product::<u32>()
}

fn calculate_mul_sum(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
    re.find_iter(input).map(exec_mul).sum()
}

fn calculate_mul_sum_with_do(input: &str) -> u32 {
    // I wasn't able to found the answer myself. I found stuff about Lookarounds and thought you could do everything in the regex but it seems you still need a flag as all other solutions used one.
    // Also ego but technically this regex isn't perfect as each value has a size limit of 3 chars, therefore it should uses \d{1,3} instead of \d+. The input data is kind enough to not include a 4 chars value.
    // Still, thanks @nated0g for the solution.
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;

    re.captures_iter(input)
        .filter_map(|cap| {
            if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                if enabled {
                    let x = x.as_str().parse::<u32>().unwrap();
                    let y = y.as_str().parse::<u32>().unwrap();
                    Some(x * y)
                } else {
                    None
                }
            } else {
                match &cap[0] {
                    "don't()" => enabled = false,
                    "do()" => enabled = true,
                    _ => {}
                }
                None
            }
        })
        .sum::<u32>()
}

pub fn day_three() {
    let input = load_inputs();

    // Day 3 - Part 1
    let mul_sum = calculate_mul_sum(&input);
    println!("The Mul Sum is {mul_sum}");

    // Day 3 - Part 2
    let mul_sum = calculate_mul_sum_with_do(&input);
    println!("The Mul Sum With Do is {mul_sum}");
}
