use std::collections::VecDeque;

fn run(skip: usize, max: usize) -> VecDeque<usize> {
    let mut deque = VecDeque::with_capacity(max);
    deque.insert(0, 0);
    for i in 1..=max {
        let rot = (skip + 1) % deque.len();
        if rot != 0 {
            deque.rotate_left(rot);
        }
        deque.insert(0, i);
    }
    deque
}

fn p1(skip: usize) -> usize {
    run(skip, 2017)[1]
}

fn p2(skip: usize) -> usize {
    let dq = run(skip, 50_000_000);
    *dq.iter().skip_while(|&t| *t != 0).skip(1).next().unwrap()
}

fn main() -> anyhow::Result<()> {
    let input: usize = std::fs::read_to_string("input")?.trim().parse().unwrap();
    dbg!(p1(input));
    dbg!(p2(input));
    Ok(())
}

#[test]
fn t1() {
    assert_eq!(p1(3), 638);
}
