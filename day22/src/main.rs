use std::collections::HashMap;

fn run(input: &str, bursts: usize, p2: bool) -> usize {
    let (infected_state, states) = if p2 { (2, 4) } else { (1, 2) };
    let mut grid: HashMap<(isize, isize), usize> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                Some(((x as isize, y as isize), infected_state)).filter(|_| c == '#')
            })
        })
        .collect();
    let mid = input.lines().count() as isize / 2;
    let (mut x, mut y) = (mid, mid);
    let (mut dx, mut dy) = (0, -1);
    let mut infectation = 0;
    for _ in 0..bursts {
        let entry = grid.entry((x, y)).or_default();
        (dx, dy) = match (p2, *entry) {
            (_, 0) => (dy, -dx),
            (false, 1) | (true, 2) => (-dy, dx),
            (true, 1) => (dx, dy),
            (true, 3) => (-dx, -dy),
            _ => panic!(),
        };
        *entry = (*entry + 1) % states;
        if *entry == infected_state {
            infectation += 1;
        }
        (x, y) = (x + dx, y + dy);
    }
    infectation
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(run(&input, 10000, false));
    dbg!(run(&input, 10000000, true));
    Ok(())
}

#[test]
fn t1() {
    assert_eq!(run("..#\n#..\n...", 7, false), 5);
}

#[test]
fn t2() {
    assert_eq!(run("..#\n#..\n...", 5, true), 26);
}
