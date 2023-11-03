use itertools::Itertools;
use pathfinding::directed::strongly_connected_components::*;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let links: HashMap<usize, Vec<usize>> = input
        .lines()
        .map(|line| {
            let (from, tos) = line.split_once("<->").unwrap();
            let from: usize = from.trim().parse().unwrap();
            let tos = tos.split(", ").map(|s| s.trim().parse().unwrap()).collect();
            (from, tos)
        })
        .collect();
    let reachable = strongly_connected_component(&0, |n| links[n].iter().copied());
    dbg!(reachable.len());
    let nodes = links.keys().copied().collect_vec();
    let components = strongly_connected_components(&nodes, |n| links[n].iter().copied());
    dbg!(components.len());
    Ok(())
}
