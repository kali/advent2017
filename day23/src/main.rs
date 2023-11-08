use itertools::Itertools;

extern "C" { fn func(mode: i32) -> i32; }

fn run_one(ops: &[&str]) -> isize {
    let mut pc = 0isize;
    let mut registers = vec![0isize; 8];
    let mut mul = 0;

    while let Some(op) = ops.get(pc as usize) {
        let tokens = op.split_whitespace().collect_vec();
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
        match tokens[0] {
            "set" => r!(1) = v!(2),
            "sub" => r!(1) -= v!(2),
            "mul" => {
                r!(1) *= v!(2);
                mul += 1
            }
            "jnz" if v!(1) != 0 => pc += v!(2) - 1,
            _ => (),
        }
        pc += 1;
    }
    mul
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let ops = input.lines().collect_vec();
    dbg!(run_one(&ops));
    dbg!(unsafe { func(0) });
    dbg!(unsafe { func(1) });
    Ok(())
}
