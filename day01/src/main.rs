use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let p1 = input
        .bytes()
        .circular_tuple_windows()
        .filter(|(a, b)| a == b)
        .map(|(a, _)| (a - b'0') as usize)
        .sum::<usize>();
    dbg!(p1);
    let input = input.as_bytes();
    let mut p2 = 0;
    for pos in 0..input.len() / 2 {
        if input[pos] == input[pos + input.len() / 2] {
            p2 += 2 * (input[pos] - b'0') as i32;
        }
    }
    dbg!(p2);
    Ok(())
}
