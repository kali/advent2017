use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let &[a0, b0] = &*input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect_vec()
    else {
        anyhow::bail!("input")
    };
    let (mut a, mut b) = (a0, b0);
    let mut p1 = 0;
    for _ in 0..40_000_000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        if (a ^ b) & 0xffff == 0 {
            p1 += 1
        }
    }
    dbg!(p1);
    let (mut a, mut b) = (a0, b0);
    let mut p2 = 0;
    for _ in 0..5_000_000 {
        loop {
            a = (a * 16807) % 2147483647;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = (b * 48271) % 2147483647;
            if b % 8 == 0 {
                break;
            }
        }
        if (a ^ b) & 0xffff == 0 {
            p2 += 1
        }
    }
    dbg!(p2);
    Ok(())
}
