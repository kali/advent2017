fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|s| {
            let (depth, range) = s.split_once(": ").unwrap();
            let depth: usize = depth.parse().unwrap();
            let range: usize = range.parse().unwrap();
            (depth, range)
        })
        .collect()
}

fn severity(layers: &[(usize, usize)]) -> usize {
    let mut severity = 0;
    for (depth, range) in layers {
        if depth % (2 * (range - 1)) == 0 {
            severity += depth * range;
        }
    }
    severity
}

fn delay(layers: &[(usize, usize)]) -> usize {
    'd: for delay in 0.. {
        for (depth, range) in layers {
            if (depth + delay) % (2 * (range - 1)) == 0 {
                continue 'd
            }
        }
        return delay
    }
    unreachable!()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let layers = parse(&input);
    let severity = severity(&layers);
    dbg!(severity);
    let delay = delay(&layers);
    dbg!(delay);
    Ok(())
}

#[test]
fn t1() {
    let layers = parse("0: 3\n1: 2\n4: 4\n6: 4");
    let sev = severity(&layers);
    assert_eq!(sev, 24);
}

#[test]
fn t2() {
    let layers = parse("0: 3\n1: 2\n4: 4\n6: 4");
    let delay = delay(&layers);
    assert_eq!(delay, 10);
}
