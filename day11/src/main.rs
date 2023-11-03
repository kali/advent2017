use itertools::Itertools;
use std::collections::HashMap;

fn rule<'s>(steps: &mut HashMap<&'s str, usize>, rule: &'s str) {
    let (from, to) = rule.split_once("->").unwrap();
    let m = from
        .split(",")
        .map(|f| steps.get(f).copied().unwrap_or_default())
        .min()
        .unwrap();
    from.split(",")
        .for_each(|f| *steps.entry(f).or_insert(0) -= m);
    if !to.is_empty() {
        to.split(",")
            .for_each(|f| *steps.entry(f).or_insert(0) += m);
    }
}

fn distance(path: &[&str]) -> usize {
    let mut steps: HashMap<&str, usize> = path.iter().copied().counts_by(|x| x);

    loop {
        let len = steps.values().sum::<usize>();
        rule(&mut steps, "n,s->");
        rule(&mut steps, "ne,sw->");
        rule(&mut steps, "nw,se->");
        rule(&mut steps, "nw,ne->n");
        rule(&mut steps, "n,sw->nw");
        rule(&mut steps, "nw,s->sw");
        rule(&mut steps, "sw,se->s");
        rule(&mut steps, "s,ne->se");
        rule(&mut steps, "se,n->ne");
        let new_len = steps.values().sum::<usize>();
        if new_len == len {
            break;
        }
    }
    steps.values().sum::<usize>()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let steps = input.split(",").collect_vec();
    dbg!(distance(&steps));
    let p2 = (0..steps.len())
        .map(|l| distance(&steps[..l]))
        .max()
        .unwrap();
    dbg!(p2);
    Ok(())
}
