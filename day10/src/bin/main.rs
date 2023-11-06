use day10::*;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();

    let numbers: Vec<u8> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let chain = hash::<256>(&*numbers, 1);
    dbg!(chain[0] * chain[1]);

    let hash = knot_hash(input.as_bytes());
    for x in hash.to_be_bytes() {
        print!("{x:02x}");
    }
    println!();

    Ok(())
}

