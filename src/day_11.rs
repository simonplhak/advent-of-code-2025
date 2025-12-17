use anyhow::Result;
use std::{collections::HashMap, path::Path};

use crate::utils::read_lines;

pub fn task_01(data_path: &Path) -> Result<String> {
    let graph = load_graph(data_path)?;
    let mut cache = HashMap::new();
    let total_paths = count_paths(&graph, "you", "out", &mut cache);
    Ok(format!("Answer: {}", total_paths))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let graph = load_graph(data_path)?;
    let mut cache = HashMap::new();
    let paths_fft2dac = count_paths(&graph, "fft", "dac", &mut cache);
    let (x1, x2, seg_b) = if paths_fft2dac > 0 {
        ("fft".to_string(), "dac".to_string(), paths_fft2dac)
    } else {
        let paths_dac2fft = count_paths(&graph, "dac", "fft", &mut cache);
        assert!(paths_dac2fft > 0);
        ("dac".to_string(), "fft".to_string(), paths_dac2fft)
    };
    let seg_a = count_paths(&graph, "svr", &x1, &mut cache);
    let seg_c = count_paths(&graph, &x2, "out", &mut cache);
    Ok(format!("Answer: {}", seg_a * seg_b * seg_c))
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    cache: &mut HashMap<(String, String), usize>,
) -> usize {
    let key = (current.to_string(), target.to_string());
    if current == target {
        cache.insert(key, 1);
        return 1;
    }
    if let Some(paths) = cache.get(&key) {
        return *paths;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(&key.0) {
        for neighbor in neighbors {
            total += count_paths(graph, neighbor, target, cache);
        }
    }
    cache.insert(key, total);
    total
}

fn load_graph(data_path: &Path) -> Result<HashMap<String, Vec<String>>> {
    let lines = read_lines(data_path)?;
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let split = line.split(":").collect::<Vec<_>>();
        assert_eq!(split.len(), 2);
        let from = split[0].trim().to_string();
        let to = split[1]
            .split_whitespace()
            .map(|to| to.trim().to_string())
            .collect::<Vec<_>>();
        let inserted = graph.insert(from, to);
        assert!(inserted.is_none());
    }
    Ok(graph)
}
