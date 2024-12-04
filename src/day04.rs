use crate::utils::read_input;

pub fn run_part_a(filename: &str) {
    let input = read_input(&filename).unwrap();

    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (n, m) = (matrix.len(), matrix[0].len());

    let directions = vec![
        vec![0, -1],  // left
        vec![0, 1],   // right
        vec![-1, 0],  // up
        vec![1, 0],   // down
        vec![-1, -1], // up-left
        vec![-1, 1],  // up-right
        vec![1, -1],  // down-left
        vec![1, 1],   // down-right
    ];

    let mut found = 0;
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 'X' {
                for direction in &directions {
                    found += if search_xmas(i as i32, j as i32, direction.clone(), &matrix) { 1 } else { 0 };
                }
            }
        }
    }
    println!("{}", found);
}

pub fn run_part_b(filename: &str) {
    let input = read_input(&filename).unwrap();

    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (n, m) = (matrix.len(), matrix[0].len());

    let mut found = 0;
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 'A' {
                found += if search_x_mas(i as i32, j as i32, &matrix) { 1 } else { 0 };
            }
        }
    }
    println!("{}", found);
}

fn search_xmas(i: i32, j: i32, direction: Vec<i32>, matrix: &Vec<Vec<char>>) -> bool {
    let (n, m) = (matrix.len() as i32, matrix[0].len() as i32);

    let is = |i: i32, j: i32, c: char| {
        i >= 0 && i < n && j >= 0 && j < m && matrix[i as usize][j as usize] == c
    };

    let (di, dj) = (direction[0], direction[1]);
    return is(i + di, j + dj, 'M') && is(i + 2 * di, j + 2 * dj, 'A') && is(i + 3 * di, j + 3 * dj, 'S');
}

fn search_x_mas(i: i32, j: i32, matrix: &Vec<Vec<char>>) -> bool {
    let (n, m) = (matrix.len() as i32, matrix[0].len() as i32);

    let is = |i: i32, j: i32, c: char| {
        i >= 0 && i < n && j >= 0 && j < m && matrix[i as usize][j as usize] == c
    };

    let (up, down, left, right) = (i - 1, i + 1, j - 1, j + 1);

    if is(up, left, 'M') && is(up, right, 'M') && is(down, left, 'S') && is (down, right, 'S') {
        return true;
    }

    if is(up, right, 'M') && is(down, right, 'M') && is(down, left, 'S') && is (up, left, 'S') {
        return true;
    }

    if is(down, right, 'M') && is(down, left, 'M') && is(up, left, 'S') && is (up, right, 'S') {
        return true;
    }

    if is(down, left, 'M') && is(up, left, 'M') && is(up, right, 'S') && is (down, right, 'S') {
        return true;
    }

    return false;
}
