use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Inst {
    dst: String,
    is_dec: bool,
    by: isize,
    src: String,
    comp: String,
    bound: isize,
}

impl FromStr for Inst {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [dst, inc_or_dec, by, _if, src, comp, bound] = &*s.split_whitespace().collect_vec()
        else {
            panic!("cant parse \"{}\"", s);
        };
        Ok(Inst {
            dst: dst.to_string(),
            is_dec: inc_or_dec == &"dec",
            by: by.parse()?,
            src: src.to_string(),
            comp: comp.to_string(),
            bound: bound.parse()?,
        })
    }
}

impl Inst {
    fn run(&self, regs: &mut HashMap<String, isize>) {
        let src_value = regs.get(&self.src).copied().unwrap_or(0);
        let cond = match &*self.comp {
            "==" => src_value == self.bound,
            "!=" => src_value != self.bound,
            "<=" => src_value <= self.bound,
            ">=" => src_value >= self.bound,
            "<" => src_value < self.bound,
            ">" => src_value > self.bound,
            _ => panic!(),
        };
        if cond {
            if self.is_dec {
                *regs.entry(self.dst.clone()).or_insert(0) -= self.by;
            } else {
                *regs.entry(self.dst.clone()).or_insert(0) += self.by;
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let prog: Vec<Inst> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut regs = HashMap::default();
    prog.iter().for_each(|i| i.run(&mut regs));
    let p1 = regs.values().max().unwrap();
    dbg!(p1);
    let mut regs = HashMap::default();
    let mut all_time_max = 0;
    for i in prog {
        i.run(&mut regs);
        all_time_max = all_time_max.max(*regs.values().max().unwrap())
    }
    dbg!(all_time_max);
    Ok(())
}
