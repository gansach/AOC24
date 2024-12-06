use crate::utils::read_input;
use std::collections::{HashSet, VecDeque};

pub fn run_part_a(filename: &str) {
    let input = read_input(&filename).unwrap();

    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (n, m) = (matrix.len(), matrix[0].len());

    let mut visited = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == '^' {
                let mut steps = 0;
                if is_cycle(i as i32, j as i32, &matrix, 0, &mut visited, &mut steps, n as i32, m as i32) {
                    break;
                }
            }
        }
    }

    println!("{}", visited.len());
}

pub fn run_part_b(filename: &str) {
    let input = read_input(&filename).unwrap();

    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (n, m) = (matrix.len(), matrix[0].len());

    let (start_i, start_j) = {
        let mut coords = (0, 0);
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == '^' {
                    coords = (i as i32, j as i32);
                    matrix[i][j] = '.';
                }
            }
        }
        coords
    };

    let mut leads_to_cycle = 0;
    let mut visited = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == '.' {
                println!("{} {}", i, j);

                // Brute force every possibility
                matrix[i][j] = '#';
                let mut steps = 0;
                if is_cycle(start_i as i32, start_j as i32, &matrix, 0, &mut visited, &mut steps, n as i32, m as i32) {
                    leads_to_cycle += 1;
                }
                matrix[i][j] = '.';
            }
        }
    }

    println!("{}", leads_to_cycle);
}

fn is_cycle(
    i: i32,
    j: i32,
    matrix: &Vec<Vec<char>>,
    direction: usize,
    visited: &mut HashSet<(i32, i32)>,
    steps: &mut i32,
    n: i32,
    m: i32,
) -> bool {
    let is_inside = |i: i32, j: i32| {
        i >= 0 && i < n && j >= 0 && j < m
    };

    let mut stack = VecDeque::new();
    stack.push_back((i, j, direction, 0));

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)]; // UP, RIGHT, DOWN, LEFT

    while let Some((i, j, direction, ..)) = stack.pop_back() {
        // Absolute brute force
        if *steps > 4 * n * m {
            return true;
        }

        visited.insert((i, j));
        let (di, dj) = directions[direction];
        let ni = i + di;
        let nj = j + dj;

        if !is_inside(ni, nj) {
            continue;
        }

        *steps += 1;

        if matrix[ni as usize][nj as usize] == '#' {
            let next_direction = (direction + 1) % 4;
            stack.push_back((i, j, next_direction, *steps));
        } else {
            stack.push_back((ni, nj, direction, *steps));
        }
    }

    return false;
}


