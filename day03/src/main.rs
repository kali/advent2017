use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut address_to_coordinates: Vec<(i32, i32)> = vec![(0, 0), (0, 0), (1, 0)];
    let input: usize = input.parse()?;
    for _ in 3..=input {
        let &(x, y) = address_to_coordinates.last().unwrap();
        let (x, y) = if y < 0 && x <= -y && x >= y {
            (x + 1, y)
        } else if x > 0 && y < x {
            (x, y + 1)
        } else if y > 0 && x > -y {
            (x - 1, y)
        } else if x < 0 && y > x {
            (x, y - 1)
        } else {
            panic!()
        };
        address_to_coordinates.push((x, y));
    }
    let (x, y) = address_to_coordinates[input];
    let p1 = x.abs() + y.abs();
    dbg!(p1);
    let index: HashMap<(i32, i32), usize> = address_to_coordinates
        .iter()
        .enumerate()
        .map(|(ix, &co)| (co, ix))
        .collect();
    let mut sums = vec![0, 1];
    for i in 2..{
        let (x0, y0) = address_to_coordinates[i];
        let mut sum = 0;
        for (dx, dy) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let (xp, yp) = (x0 + dx, y0 + dy);
            if let Some(&ip) = index.get(&(xp, yp)) {
                sum += sums.get(ip).copied().unwrap_or(0);
            }
        }
        sums.push(sum);
        if sum > input {
            dbg!(sum);
            break;
        }
    }
    Ok(())
}
