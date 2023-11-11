use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let components: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.split('/')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b)| (a.min(b), a.max(b)))
        .sorted()
        .collect_vec();
    let mut p1 = 0;
    let mut p2 = (0, 0);
    for bridge in pathfinding::directed::bfs::bfs_reach(vec![0], |path| {
        components
            .iter()
            .filter(|(a, b)| a == path.last().unwrap() || b == path.last().unwrap())
            .filter(|(a, b)| {
                !path
                    .iter()
                    .tuple_windows()
                    .any(|(a0, b0)| (a, b) == (a0, b0) || (a, b) == (b0, a0))
            })
            .map(|(a, b)| {
                let mut path = path.clone();
                path.push(if path.last().unwrap() == a { *b } else { *a });
                path
            })
            .collect_vec()
    }) {
        let strength = 2 * bridge.iter().sum::<usize>() - bridge[bridge.len() - 1];
        p1 = p1.max(strength);
        p2 = p2.max((bridge.len(), strength));
    }
    dbg!(p1);
    dbg!(p2.1);
    Ok(())
}
