use itertools::Itertools;

fn hash(n: usize, input: &[usize], rounds: usize) -> Vec<usize> {
    let mut chain = (0..).take(n).collect_vec();
    let mut position = 0;
    let mut skip = 0;
    for _ in 0..rounds {
        for &l in input {
            for j in 0..l / 2 {
                chain.swap((position + j) % n, (position + l - j - 1) % n);
            }
            position += (l + skip) % n;
            skip += 1;
        }
    }
    chain
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();

    let numbers: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let chain = hash(256, &*numbers, 1);
    dbg!(chain[0] * chain[1]);

    let mut msg = input.bytes().map(|b| b as usize).collect_vec();
    msg.extend([17, 31, 73, 47, 23].into_iter());

    let hash = hash(256, &msg, 64);
    let dense = hash
        .chunks(16)
        .map(|block| block.iter().copied().reduce(|a, b| a ^ b).unwrap() as u8)
        .collect_vec();
    for x in dense {
        print!("{x:02x}");
    }
    println!();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let chain = hash(5, &[3, 4, 1, 5], 1);
        assert_eq!(chain[0] * chain[1], 12);
    }
}
