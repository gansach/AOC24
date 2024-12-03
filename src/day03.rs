use crate::utils::read_input;
use regex::Regex;

pub fn run_part_a(filename: &str) {
    let input = read_input(&filename).unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum_of_products: i64 = 0;
    for (_, [x, y]) in re.captures_iter(&input).map(|c| c.extract()) {
        sum_of_products += x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap();
    }
    println!("{}", sum_of_products);
}

pub fn run_part_b(filename: &str) {
    let input = read_input(&filename).unwrap();

    let n = input.len();
    let mut enabled = true;
    let mut sum_of_products = 0;

    let mut i = 0;
    while i < n {
        if &input[i..i+1] == "d" {
            if i + 6 <= n && &input[i..i+6] == "don't()" {
                enabled = false;
                i += 6;
                continue;
            }
            else if i + 3 <= n && &input[i..i+3] == "do()" {
                enabled = true;
                i += 3;
                continue;
            }
        }

        if enabled && i + 4 <= input.len() && &input[i..i+4] == "mul(" {
            i += 4;
            let mut x = String::new();
            while i < input.len() && input.chars().nth(i).unwrap().is_digit(10) {
                x.push(input.chars().nth(i).unwrap());
                i += 1;
            }

            if i < input.len() && &input[i..i+1] != "," {
                continue;
            }
            i += 1;

            let mut y = String::new();
            while i < input.len() && input.chars().nth(i).unwrap().is_digit(10) {
                y.push(input.chars().nth(i).unwrap());
                i += 1;
            }

            if i < input.len() && &input[i..i+1] != ")" {
                continue;
            }
            i += 1;

            let x_num: i32 = x.parse().unwrap();
            let y_num: i32 = y.parse().unwrap();
            sum_of_products += x_num * y_num;
            continue;
        }

        i += 1;
    }

    println!("{}", sum_of_products);
}
