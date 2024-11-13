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

    loop {
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression: &str = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition: i32 = left_value + right_value;

        expression = expression.replace(cap_expression, &addition.to_string());
    }

    // Operations

    // Show result
    println!("Result: {}", expression)
}
