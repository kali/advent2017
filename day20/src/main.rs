use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let start: Vec<[isize; 9]> = input
        .lines()
        .map(|line| {
            array_init::from_iter(
                line.replace(['p', '=', '<', '>', ',', 'v', 'a'], " ")
                    .split_whitespace()
                    .map(|x| x.parse::<isize>().unwrap()),
            )
            .unwrap()
        })
        .collect();

    let p1 = start
        .iter()
        .map(|star| (star[6].abs() + star[7].abs() + star[8].abs()))
        .position_min()
        .unwrap();
    dbg!(p1);

    let mut state = start.clone();
    loop {
        for star in &mut state {
            for c in [0, 1, 2] {
                star[3 + c] += star[6 + c];
                star[c] += star[3 + c];
            }
        }
        (0..state.len())
            .tuple_combinations()
            .filter(|(a, b)| state[*a][0..3] == state[*b][0..3])
            .flat_map(|(a, b)| [a, b].into_iter())
            .sorted()
            .rev()
            .dedup()
            .for_each(|c| {
                state.remove(c);
            });
        if state.iter().tuple_combinations().all(|(a, b)| {
            let dist = a.iter().zip(b.iter()).map(|(a, b)| a - b).collect_vec();
            let r = (dist[0].signum() == dist[3].signum() || dist[3] == 0)
                && (dist[0].signum() == dist[6].signum() || dist[6] == 0)
                && (dist[1].signum() == dist[4].signum() || dist[4] == 0)
                && (dist[1].signum() == dist[7].signum() || dist[7] == 0)
                && (dist[2].signum() == dist[5].signum() || dist[5] == 0)
                && (dist[2].signum() == dist[8].signum() || dist[8] == 0);
            r
        }) {
            break;
        }
    }
    dbg!(state.len());

    Ok(())
}
