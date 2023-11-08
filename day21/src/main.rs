use std::collections::HashMap;

use itertools::Itertools;

fn generate(i: [bool; 9]) -> [[bool; 9]; 8] {
    let r = |[tl, t, tr, l, c, r, bl, b, br]: [bool; 9]| [bl, l, tl, b, c, t, br, r, tr];
    let f = |[tl, t, tr, l, c, r, bl, b, br]: [bool; 9]| [tr, t, tl, r, c, l, br, b, bl];
    [
        i,
        r(i),
        r(r(i)),
        r(r(r(i))),
        f(i),
        r(f(i)),
        r(r(f(i))),
        r(r(r(f(i)))),
    ]
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut pats = HashMap::<Vec<bool>, Vec<bool>>::default();
    for line in input.lines() {
        let (a, b) = line.split_once(" => ").unwrap();
        let a = a
            .chars()
            .filter(|c| *c != '/')
            .map(|c| c == '#')
            .collect_vec();
        let b = b
            .chars()
            .filter(|c| *c != '/')
            .map(|c| c == '#')
            .collect_vec();
        if let &[tl, tr, bl, br] = &*a {
            for [tl, _, tr, _, _, _, bl, _, br] in
                generate([tl, false, tr, false, false, false, bl, false, br])
            {
                pats.insert(vec![tl, tr, bl, br], b.clone());
            }
        } else {
            let a = array_init::from_iter(a.into_iter()).unwrap();
            for pat in generate(a) {
                pats.insert(pat.to_vec(), b.clone());
            }
        }
    }
    let mut state = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    for i in 1..=18 {
        let size = state.len();
        let (f, t) = if size % 2 == 0 { (2, 3) } else { (3, 4) };
        let mut new_state = vec![vec!(false; size * t / f); size * t / f];
        let old = &state;
        for x in 0..size / f {
            for y in 0..size / f {
                let pat = (0..f)
                    .flat_map(|dy| (0..f).map(move |dx| old[y * f + dy][x * f + dx]))
                    .collect_vec();
                let into = &pats[&pat];
                for dx in 0..t {
                    for dy in 0..t {
                        new_state[t * y + dy][t * x + dx] = into[t * dy + dx]
                    }
                }
            }
        }
        state = new_state;
        if i == 5 || i == 18 {
            let pixels = state.iter().flat_map(|l| l.iter()).filter(|x| **x).count();
            dbg!(pixels);
        }
    }
    Ok(())
}
