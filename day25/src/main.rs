use std::collections::HashMap;

use anyhow::Result;
use scan_fmt::scan_fmt;

type State = (Action, Action); // 0 1
type Action = (isize, bool, char); // write, to_right, next

fn parse_action<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<Action> {
    let write = scan_fmt!(lines.next().unwrap(), "- Write the value {}.", isize)?;
    let right = lines.next().unwrap().trim() == "- Move one slot to the right.";
    let next = scan_fmt!(lines.next().unwrap(), "- Continue with state {}.", char)?;
    Ok((write, right, next))
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?;
    let mut lines = input.trim().lines().filter(|l| l.len() > 0);
    let start = scan_fmt!(lines.next().unwrap(), "Begin in state {}.", char)?;
    let stop = scan_fmt!(
        lines.next().unwrap(),
        "Perform a diagnostic checksum after {} steps.",
        usize
    )?;

    let mut prog = HashMap::<char, State>::default();

    while let Some(line) = lines.next() {
        let letter = scan_fmt!(line, "In state {}:", char)?;
        let _ = lines.next().unwrap();
        let a0 = parse_action(&mut lines)?;
        let _ = lines.next().unwrap();
        let a1 = parse_action(&mut lines)?;
        prog.insert(letter, (a0, a1));
    }

    let mut tape = HashMap::<isize, isize>::default();
    let mut position = 0;
    let mut state = start;
    for _ in 0..stop {
        let cell = tape.entry(position).or_default();
        let (write, right, next) = if *cell == 0 {
            prog[&state].0
        } else {
            prog[&state].1
        };
        *cell = write;
        position += if right { 1 } else { -1 };
        state = next;
    }

    let p1 = tape.values().sum::<isize>();
    dbg!(p1);

    Ok(())
}
