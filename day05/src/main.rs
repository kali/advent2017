fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let input: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut prog = input.clone();
    let mut pc = 0isize;
    for step in 1.. {
        prog[pc as usize] += 1;
        pc = pc + prog[pc as usize] - 1;
        if pc < 0 || pc as usize >= prog.len() {
            dbg!(step);
            break;
        }
    }
    let mut prog = input.clone();
    let mut pc = 0isize;
    for step in 1.. {
        let jump = prog[pc as usize];
        prog[pc as usize] += if jump >= 3 { -1 } else { 1 };
        pc += jump;
        if pc < 0 || pc as usize >= prog.len() {
            dbg!(step);
            break;
        }
    }
    Ok(())
}
