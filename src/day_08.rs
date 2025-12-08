use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;
use std::vec;

use crate::utils::Point3D;

const NUM_LARGEST_CONNECTIONS: usize = 3;

struct Connection {
    node_a: usize,
    node_b: usize,
}

fn dists(map: &[Point3D]) -> Vec<Vec<f64>> {
    let mut dists = vec![vec![0.0; map.len()]; map.len()];
    for x in 0..map.len() {
        let point_a = &map[x];
        for y in (x + 1)..map.len() {
            let point_b = &map[y];
            let dist = point_a.l2(point_b);
            dists[x][y] = dist;
            dists[y][x] = dist;
        }
    }
    dists
}

#[allow(clippy::needless_range_loop)]
fn min_dist(dists: &[Vec<f64>]) -> Connection {
    let mut min_dist = f64::MAX;
    let mut min_pair = Connection {
        node_a: 0,
        node_b: 0,
    };
    for x in 0..dists.len() {
        for y in x + 1..dists.len() {
            if dists[x][y] < min_dist {
                min_dist = dists[x][y];
                min_pair = Connection {
                    node_a: x,
                    node_b: y,
                };
            }
        }
    }
    assert!(min_dist < f64::MAX);
    assert!(min_dist > 0.0);
    min_pair
}

pub fn task_01(data_path: &Path, num_connections: usize) -> Result<String> {
    let map = load_map(data_path)?;
    let mut dists = dists(&map);
    let mut connections: Vec<Vec<usize>> = Vec::with_capacity(num_connections);
    let find_connection = |connections: &[Vec<usize>], node| {
        connections
            .iter()
            .enumerate()
            .find(|(_, conn)| conn.contains(&node))
            .map(|(idx, _)| idx)
    };
    for _ in 0..num_connections {
        let min_pair = min_dist(&dists);
        let conn_link_x = find_connection(&connections, min_pair.node_a);
        let conn_link_y = find_connection(&connections, min_pair.node_b);
        match (conn_link_x, conn_link_y) {
            (Some(conn_a_idx), Some(conn_b_idx)) => {
                if conn_a_idx != conn_b_idx {
                    // ensure that conn_b > conn_a to not mess up indices when removing
                    let ((node_a, conn_a_idx), (node_b, conn_b_idx)) = if conn_a_idx > conn_b_idx {
                        ((min_pair.node_b, conn_b_idx), (min_pair.node_a, conn_a_idx))
                    } else {
                        ((min_pair.node_a, conn_a_idx), (min_pair.node_b, conn_b_idx))
                    };
                    let mut conn_b = connections.remove(conn_b_idx);
                    let conn_a = &mut connections[conn_a_idx];

                    assert!(conn_a.contains(&node_a),);
                    assert!(conn_b.contains(&node_b),);
                    conn_a.append(&mut conn_b);
                }
            }
            (Some(conn_idx), None) => {
                let conn = &mut connections[conn_idx];
                conn.push(min_pair.node_b);
            }
            (None, Some(conn_idx)) => {
                let conn = &mut connections[conn_idx];
                conn.push(min_pair.node_a);
            }
            (None, None) => {
                connections.push(vec![min_pair.node_a, min_pair.node_b]);
            }
        }
        dists[min_pair.node_a][min_pair.node_b] = f64::MAX;
        dists[min_pair.node_b][min_pair.node_a] = f64::MAX;
    }
    assert!(connections.len() >= NUM_LARGEST_CONNECTIONS);
    let mut unique_nodes = HashSet::new();
    for conn in &connections {
        for node in conn {
            assert!(unique_nodes.insert(*node));
        }
    }
    let mut connections_len = connections
        .iter()
        .map(|conn| conn.len())
        .collect::<Vec<_>>();
    connections_len.sort_by(|a, b| a.cmp(b).reverse());
    let answer: usize = connections_len
        .iter()
        .take(NUM_LARGEST_CONNECTIONS)
        .product();

    Ok(format!("Answer: {}", answer))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let map = load_map(data_path)?;

    let mut dists = dists(&map);
    let mut connections: Vec<Vec<usize>> = Vec::new();
    let find_connection = |connections: &[Vec<usize>], node| {
        connections
            .iter()
            .enumerate()
            .find(|(_, conn)| conn.contains(&node))
            .map(|(idx, _)| idx)
    };
    let mut curr_pair = min_dist(&dists);

    while connections.len() != 1 || connections[0].len() < map.len() {
        curr_pair = min_dist(&dists);
        let conn_link_x = find_connection(&connections, curr_pair.node_a);
        let conn_link_y = find_connection(&connections, curr_pair.node_b);
        match (conn_link_x, conn_link_y) {
            (Some(conn_a_idx), Some(conn_b_idx)) => {
                if conn_a_idx != conn_b_idx {
                    // ensure that conn_b > conn_a to not mess up indices when removing
                    let ((node_a, conn_a_idx), (node_b, conn_b_idx)) = if conn_a_idx > conn_b_idx {
                        (
                            (curr_pair.node_b, conn_b_idx),
                            (curr_pair.node_a, conn_a_idx),
                        )
                    } else {
                        (
                            (curr_pair.node_a, conn_a_idx),
                            (curr_pair.node_b, conn_b_idx),
                        )
                    };
                    let mut conn_b = connections.remove(conn_b_idx);
                    let conn_a = &mut connections[conn_a_idx];

                    assert!(conn_a.contains(&node_a),);
                    assert!(conn_b.contains(&node_b),);
                    conn_a.append(&mut conn_b);
                }
            }
            (Some(conn_idx), None) => {
                let conn = &mut connections[conn_idx];
                conn.push(curr_pair.node_b);
            }
            (None, Some(conn_idx)) => {
                let conn = &mut connections[conn_idx];
                conn.push(curr_pair.node_a);
            }
            (None, None) => {
                connections.push(vec![curr_pair.node_a, curr_pair.node_b]);
            }
        }
        dists[curr_pair.node_a][curr_pair.node_b] = f64::MAX;
        dists[curr_pair.node_b][curr_pair.node_a] = f64::MAX;
    }
    let mut unique_nodes = HashSet::new();
    for conn in &connections {
        for node in conn {
            assert!(unique_nodes.insert(*node));
        }
    }
    let answer = map[curr_pair.node_a].x * map[curr_pair.node_b].x;
    Ok(format!("Answer: {}", answer))
}

fn load_map(data_path: &Path) -> Result<Vec<Point3D>> {
    let content = std::fs::read_to_string(data_path)?;
    let map = content
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            assert!(coords.len() == 3);
            Point3D {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect::<Vec<_>>();
    Ok(map)
}
