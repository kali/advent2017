fn dance(start: &[u8], steps: &str) -> Vec<u8> {
    let mut position = start.to_vec();
    for step in steps.split(",") {
        match step.as_bytes()[0] {
            b's' => {
                let n = step.trim_start_matches("s").parse::<usize>().unwrap();
                for _ in 0..n {
                    let tmp = position.pop().unwrap();
                    position.insert(0, tmp);
                }
            }
            b'x' => {
                let (a, b) = step.trim_start_matches('x').split_once('/').unwrap();
                let a: usize = a.parse().unwrap();
                let b: usize = b.parse().unwrap();
                position.swap(a, b)
            }
            b'p' => {
                let a = position
                    .iter()
                    .position(|&x| x == step.as_bytes()[1])
                    .unwrap();
                let b = position
                    .iter()
                    .position(|&x| x == step.as_bytes()[3])
                    .unwrap();
                position.swap(a, b)
            }
            _ => panic!(),
        }
    }
    position
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let p0 = b"abcdefghijklmnop";
    let p1 = dance(p0, &input);
    dbg!(std::str::from_utf8(&p1).unwrap());
    let mut p = p0.to_vec();
    let mut cycle = 0;
    loop {
        p = dance(&p, &input);
        cycle += 1;
        if p == p0 {
            break;
        }
    }
    let rem = 1_000_000_000 % cycle;
    let mut p = p0.to_vec();
    for _ in 0..rem {
        p = dance(&p, &input);
    }
    dbg!(std::str::from_utf8(&p).unwrap());
    Ok(())
}

#[test]
fn t1() {
    assert_eq!(dance(b"abcde", "s1,x3/4,pe/b"), b"baedc");
}
