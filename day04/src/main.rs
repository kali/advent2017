use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let p1 = input
        .lines()
        .filter(|line| line.split_whitespace().all_unique())
        .count();
    dbg!(p1);
    let p2 = input
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .map(|word| word.chars().sorted().collect_vec())
                .all_unique()
        })
        .count();
    dbg!(p2);
    Ok(())
}
