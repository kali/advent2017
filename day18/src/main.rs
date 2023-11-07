use itertools::Itertools;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

fn run_one(ops: &[&str], p2: Option<(usize, Receiver<isize>, Sender<isize>)>) -> isize {
    let mut pc = 0isize;
    let mut snd = 0isize;
    let mut registers = vec![0isize; 26];
    if let Some(p2) = &p2 {
        registers[(b'p' - b'a') as usize] = p2.0 as isize;
    }

    loop {
        let tokens = ops[pc as usize].split_whitespace().collect_vec();
        macro_rules! r {
            ($r: expr) => {
                registers[(tokens[$r].as_bytes()[0] - b'a') as usize]
            };
        }
        macro_rules! v {
            ($v: expr) => {
                tokens[$v].parse::<isize>().unwrap_or_else(|_| r!($v))
            };
        }
        match (tokens[0], p2.as_ref()) {
            ("snd", None) => snd = v!(1),
            ("snd", Some(p2)) => {
                snd += 1;
                p2.2.send(v!(1)).unwrap();
            }
            ("set", _) => r!(1) = v!(2),
            ("add", _) => r!(1) += v!(2),
            ("mul", _) => r!(1) *= v!(2),
            ("mod", _) => r!(1) %= v!(2),
            ("rcv", None) if v!(1) != 0 => return snd,
            ("rcv", Some(p2)) => match p2.1.recv_timeout(Duration::from_millis(1)) {
                Ok(it) => r!(1) = it,
                Err(_) => return snd,
            },
            ("jgz", _) if v!(1) > 0 => pc += v!(2) - 1,
            _ => (),
        }
        pc += 1;
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let ops = input.lines().collect_vec();
    dbg!(run_one(&ops, None));

    let (tx0, rx1) = channel();
    let (tx1, rx0) = channel();
    let p2 = std::thread::scope(|s| {
        s.spawn(|| run_one(&ops, Some((0, rx0, tx0))));
        s.spawn(|| run_one(&ops, Some((1, rx1, tx1)))).join()
    })
    .unwrap();

    dbg!(p2);
    Ok(())
}
