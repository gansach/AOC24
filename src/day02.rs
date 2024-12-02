use crate::utils::read_input;

pub fn run_part_a(filename: &str) {
    let input = read_input(&filename).unwrap();
    let lines = input.lines();

    let mut safe_reports = 0;
    for line in lines {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        let reversed_report = report.iter().rev().cloned().collect();

        if is_safe_report(report) || is_safe_report(reversed_report) {
            safe_reports += 1;
        }
    }
    println!("{}", safe_reports);
}

pub fn run_part_b(filename: &str) {
    let input = read_input(&filename).unwrap();
    let lines = input.lines();

    let mut almost_safe_reports = 0;
    for line in lines {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        let mut is_almost_safe = false;

        for r in generate_vectors_by_removing_one(&report) {
            if !is_almost_safe && is_safe_report(r) {
                is_almost_safe = true;
            }
        }

        let reversed_report = report.iter().rev().cloned().collect();
        for r in generate_vectors_by_removing_one(&reversed_report) {
            if !is_almost_safe && is_safe_report(r) {
                is_almost_safe = true;
            }
        }

        if is_almost_safe {
            almost_safe_reports += 1;
        }
    }
    println!("{}", almost_safe_reports);
}

fn is_safe_report(report: Vec<i32>) -> bool {
    for i in 1..report.len() {
        if report[i] <= report[i - 1] {
            return false;
        }

        let diff = report[i] - report[i - 1];
        if diff > 3 {
            return false;
        }
    }
    return true;
}

fn generate_vectors_by_removing_one<T>(vec: &Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut vecs = Vec::new();
    for i in 0..vec.len() {
        let mut new_vec = vec.clone();
        new_vec.remove(i);
        vecs.push(new_vec);
    }
    return vecs;
}
