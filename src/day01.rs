use std::collections::HashMap;

use crate::utils::read_input;

pub fn run_part_a(filename: &str) {
    let input = read_input(&filename).unwrap();

    let lines = input.lines();

    let mut x_values: Vec<i32> = Vec::new();
    let mut y_values: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        x_values.push(x);
        y_values.push(y);
    }

    x_values.sort();
    y_values.sort();

    let mut sum_of_abs_diffs = 0;
    for i in 0..x_values.len() {
        let diff = (x_values[i] - y_values[i]).abs();
        sum_of_abs_diffs += diff;
    }

    println!("{}", sum_of_abs_diffs);
}

pub fn run_part_b(filename: &str) {
    let input = read_input(&filename).unwrap();

    let lines = input.lines();

    let mut x_values: Vec<i32> = Vec::new();
    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();

        let freq = freq_map.entry(y).or_insert(0);
        *freq += 1;

        x_values.push(x);
    }

    let mut similarity_score = 0;
    for x in x_values {
        let freq = freq_map.entry(x).or_insert(0);
        similarity_score += x * (*freq);
    }

    println!("{}", similarity_score);
}
