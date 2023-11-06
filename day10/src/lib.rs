use itertools::Itertools;

pub fn hash<const N: usize>(input: &[u8], rounds: usize) -> Vec<usize> {
    let mut chain = (0..).take(N).collect_vec();
    let mut position = 0 as usize;
    let mut skip = 0 as usize;
    for _ in 0..rounds {
        for &l in input {
            let l = l as usize;
            for j in 0..l / 2 {
                chain.swap((position + j) % N, (position + l - j - 1) % N);
            }
            position += (l + skip) % N;
            skip += 1;
        }
    }
    chain
}

pub fn knot_hash(bytes: impl AsRef<[u8]>) -> u128 {
    let msg = bytes
        .as_ref()
        .iter()
        .copied()
        .chain([17, 31, 73, 47, 23].into_iter())
        .collect_vec();
    let hash = hash::<256>(&msg, 64);
    let bytes = hash
        .chunks(16)
        .map(|block| block.iter().copied().reduce(|a, b| a ^ b).unwrap() as u8)
        .collect_vec()
        .try_into()
        .unwrap();
    u128::from_be_bytes(bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let chain = hash::<5>(&[3, 4, 1, 5], 1);
        assert_eq!(chain[0] * chain[1], 12);
    }
}
