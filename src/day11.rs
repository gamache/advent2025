use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(lines: &Vec<String>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let key_and_values: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
        let key = key_and_values[0].clone();
        let values: Vec<String> = key_and_values[1]
            .split(" ")
            .map(|x| x.to_string())
            .collect();
        graph.insert(key, values);
        graph.insert(String::from("out"), vec![]);
    }

    part1(&graph);
    part2(&graph);
}

fn part1(graph: &HashMap<String, Vec<String>>) {
    let part1 = count_paths(&graph, "you", "out", &vec![]);
    println!("day 11 part 1: {}", part1);
}

fn part2(graph: &HashMap<String, Vec<String>>) {
    /*
    println!("digraph {{");
    for (key, values) in &graph {
        println!("{} -> {{ {} }}", key, values.join(" "));
    }
    println!("fft [style=filled fillcolor=yellow]");
    println!("dac [style=filled fillcolor=yellow]");
    println!("}}");
     */

    // all traffic passes through these
    let waypoints: Vec<Vec<&str>> = vec![
        vec!["svr"],
        vec!["vtw", "nxo", "log"],
        vec!["tim", "ejm", "gcp", "ony", "qad"],
        vec!["fft"],
        vec!["nno", "uur", "vpw", "idq", "dsj"],
        vec!["gzw", "oeh", "qsv", "nqc"],
        vec!["wsv", "sar", "hav", "kgc"],
        vec!["dac"],
        vec!["vms", "biw", "you"],
        vec!["out"],
    ];

    // different than waypoints around fft and dac
    let stop_ats: Vec<Vec<&str>> = vec![
        vec!["svr"],
        vec!["vtw", "nxo", "log"],
        vec!["tim", "ejm", "gcp", "ony", "qad"],
        vec!["nno", "uur", "vpw", "idq", "dsj"],
        vec!["nno", "uur", "vpw", "idq", "dsj"],
        vec!["gzw", "oeh", "qsv", "nqc"],
        vec!["wsv", "sar", "hav", "kgc"],
        vec!["vms", "biw", "you"],
        vec!["vms", "biw", "you"],
        vec!["out"],
    ];

    let mut path_segments: Vec<(&str, &str, Vec<&str>)> = vec![];
    for i in 1..waypoints.len() {
        for from in &waypoints[i - 1] {
            for to in &waypoints[i] {
                path_segments.push((from, to, stop_ats[i].clone()));
            }
        }
    }
    let path_counts: HashMap<(&str, &str), u64> = path_segments
        .into_par_iter()
        .map(|(from, to, stop)| ((from, to), count_paths(&graph, from, to, &stop)))
        .collect();

    let mut all_paths: Vec<Vec<&str>> = vec![vec![]];
    for ws in &waypoints {
        let mut new_paths: Vec<Vec<&str>> = vec![];
        for w in ws {
            for path in &all_paths {
                let mut new_path = path.clone();
                new_path.push(w);
                new_paths.push(new_path);
            }
        }
        all_paths = new_paths;
    }

    let part2: u64 = all_paths
        .into_par_iter()
        .map(|path| {
            let mut count = 1u64;
            for i in 1..path.len() {
                count *= path_counts.get(&(path[i - 1], path[i])).unwrap();
            }
            count
        })
        .sum();

    println!("day 11 part 2: {}", part2);
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    from: &str,
    to: &str,
    stop_at: &Vec<&str>,
) -> u64 {
    let mut count = 0u64;
    let mut paths = vec![vec![from.to_string()]];

    while let Some(path) = paths.pop() {
        let current = &path[path.len() - 1];
        if current == to {
            count += 1;
            continue;
        }
        if stop_at.contains(&current.as_str()) {
            continue;
        }
        if None == graph.get(current) {
            println!("current={}", current);
            println!("{:?}", graph);
        }
        for node in &graph[current] {
            let mut p = path.clone();
            p.push(node.clone());
            paths.push(p);
        }
    }

    count
}
