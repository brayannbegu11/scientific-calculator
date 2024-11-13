use std::io::stdin;

use regex::Regex;

fn main() {
    println!("Hello, world!");

    // Regex

    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    // (\d+)\s?\+\s?(\d+) -> 21 + 23

    // User Data

    println!("Please introduce your expression: ");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // Operations
    let caps = re_add.captures(expression.as_str()).unwrap();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    println!("{:?}", caps);
    println!("Left : {}", left_value);
    println!("Right: {}", right_value);

    let addition: i32 = left_value + right_value;

    // Show result
    println!("Result: {}", addition)
}
