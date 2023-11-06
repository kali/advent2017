use std::collections::HashSet;

use day10::knot_hash;
use itertools::Itertools;
use pathfinding::undirected::connected_components::*;

fn p1(key: &str) -> u32 {
    (0..=127)
        .map(|n| knot_hash(format!("{key}-{n}")))
        .map(|x| x.count_ones())
        .sum::<u32>()
}

fn p2(key: &str) -> usize {
    let used: HashSet<(isize, isize)> = (0..=127)
        .flat_map(move |y| {
            let hash = knot_hash(format!("{key}-{y}"));
            (0..=127)
                .filter(move |x| hash & (1u128 << x) != 0)
                .map(move |x| (x, y))
        })
        .collect();
    let starts = used.iter().copied().collect_vec();
    connected_components(&starts, |&(x, y)| {
        [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .into_iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(|pair| used.contains(pair))
    })
    .len()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(p1(&input));
    dbg!(p2(&input));
    Ok(())
}

#[test]
fn t1() {
    assert_eq!(p1("flqrgnkx"), 8108);
}

#[test]
fn t2() {
    assert_eq!(p2("flqrgnkx"), 1242);
}
