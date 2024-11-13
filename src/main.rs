use regex::Regex;

fn main() {
    println!("Hello, world!");

    // Regex

    let re_add = Regex::new(r"(\d+)\s*\+\s*(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s*\*\s*(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s*\/\s*(\d+)").unwrap();
    let re_subs = Regex::new(r"(\d+)\s*\-\s*(\d+)").unwrap();
    // (\d+)\s?\+\s?(\d+) -> 21 + 23

    // User Data

    println!("Please introduce your expression: ");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    //Product
    loop {
        let caps = re_mult.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression: &str = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let multiplication: i32 = left_value * right_value;

        expression = expression.replace(cap_expression, &multiplication.to_string());
    }
    // Division
    loop {
        let caps = re_div.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression: &str = caps.get(0).unwrap().as_str();
        let left_value: f64 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: f64 = caps.get(2).unwrap().as_str().parse().unwrap();

        let division: f64 = left_value / right_value;

        expression = expression.replace(cap_expression, &division.to_string());
    }

    //Addition
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
    // Substraction
    loop {
        let caps = re_subs.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression: &str = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let substraction: i32 = left_value - right_value;

        expression = expression.replace(cap_expression, &substraction.to_string());
    }

    // Show result
    println!("Result: {}", expression)
}
