use crate::utils::read_input;
use std::collections::{HashSet, HashMap, VecDeque};

pub fn run_part_a(filename: &str) {
    let input = read_input(&filename).unwrap();
    let (rules, manuals) = parse_input(&input);

    let mut sum = 0;
    for manual in manuals {
        if is_valid_manual(&manual, &rules) {
            sum += manual[manual.len() / 2].parse::<i32>().unwrap();
        }
    }
    println!("{}", sum);
}

pub fn run_part_b(filename: &str) {
    let input = read_input(&filename).unwrap();
    let (rules, manuals) = parse_input(&input);

    let mut sum = 0;
    for manual in manuals {
        if is_valid_manual(&manual, &rules) {
            continue;
        }
        let sorted_manual = topo_sort_manual(manual, &rules);
        sum += sorted_manual[sorted_manual.len() / 2].parse::<i32>().unwrap();
    }
    println!("{}", sum);
}

fn is_valid_manual(manual: &Vec<String>, rules: &HashSet<String>) -> bool {
    let mut is_valid = true;
    for i in 0..manual.len() {
        for j in (i + 1)..manual.len() {
            let key = format!("{}|{}", manual[j], manual[i]);
            if rules.contains(&key) {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            break;
        }
    }
    return is_valid;
}

fn parse_input(input: &str) -> (HashSet<String>, Vec<Vec<String>>) {
    let mut rules: HashSet<String> = HashSet::new();
    let mut manuals: Vec<Vec<String>> = Vec::new();

    let mut rules_ended = false;
    for line in input.lines() {
        if line.trim().is_empty() {
            rules_ended = true;
            continue;
        }

        if rules_ended {
            let manual_parts = line.split(',').map(|s| s.trim().to_string()).collect();
            manuals.push(manual_parts);
        } else {
            rules.insert(line.to_string());
        }
    }
    return (rules, manuals);
}

fn make_rules_graph(rules: &HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rules {
        if let Some((src, dest)) = rule.split_once('|') {
            graph.entry(src.to_string())
                 .or_insert_with(Vec::new)
                 .push(dest.to_string());
        }
    }
    return graph;
}

fn topo_sort_manual(manual: Vec<String>, rules: &HashSet<String>) -> Vec<String> {
    let mut in_degree: HashMap<String, i32> = HashMap::new();
    let mut relevant_rules: HashSet<String> = HashSet::new();

    for rule in rules {
        for page in &manual {
            if rule.starts_with(&(page.clone() + "|")) {
                let dest = rule.split('|').nth(1).unwrap().to_string();
                *in_degree.entry(dest.clone()).or_insert(0) += 1;
                in_degree.entry(page.clone()).or_insert(0);
                relevant_rules.insert(rule.clone());
            }
        }
    }

    // Perform topological sorting only on the rules relevant to the pages in the manual.
    // The overall set of rules may contain cycles, but this function isolates and sorts
    // the subset of rules applicable to the given manual pages.
    let graph = make_rules_graph(&relevant_rules);

    let mut queue: VecDeque<String> = VecDeque::new();
    for page in &manual {
        if *in_degree.get(page).unwrap_or(&0) == 0 {
            queue.push_back(page.clone());
        }
    }

    let mut topo_sort: Vec<String> = Vec::new();
    while let Some(page) = queue.pop_front() {
        if manual.contains(&page) {
            topo_sort.push(page.clone());
        }
        if let Some(neighbors) = graph.get(&page) {
            for neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }

    return topo_sort;
}
