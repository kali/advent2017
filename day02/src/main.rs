use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let numbers: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let p1: usize = numbers
        .iter()
        .map(|numbers| numbers.iter().max().unwrap() - numbers.iter().min().unwrap())
        .sum();
    dbg!(p1);

    let p2: usize = numbers
        .iter()
        .map(|numbers| {
            numbers
                .iter()
                .tuple_combinations()
                .map(|(a, b)| {
                    let big = a.max(b);
                    let small = a.min(b);
                    if big % small == 0 {
                        big / small
                    } else {
                        0
                    }
                })
                .max()
                .unwrap()
        })
        .sum();
    dbg!(p2);
    Ok(())
}
