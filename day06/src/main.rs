use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut banks: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let banks_len = banks.len();
    let mut history = HashMap::<Vec<usize>, usize>::default();
    for step in 1.. {
        let (&max, argmax) = banks
            .iter()
            .enumerate()
            .map(|(ix, n)| (n, -(ix as isize)))
            .max()
            .unwrap();
        let argmax = (-argmax) as usize;
        banks[argmax] = 0;
        for i in 0..max {
            banks[(argmax + i + 1) % banks_len] += 1;
        }
        if let Some(prev) = history.get(&banks) {
            dbg!(step, step - prev);
            break; 
        } else {
            history.insert(banks.clone(), step);
        }
    }
    Ok(())
}
