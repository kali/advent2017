fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut depth = 0;
    let mut score = 0;
    let mut cancelling = false;
    let mut garbageing = false;
    let mut garbage = 0;
    for c in input.chars() {
        if cancelling {
            cancelling = false;
            continue;
        }
        match c {
            '!' => { cancelling = true },
            '>' => { garbageing = false },
            '{' if !garbageing => {
                depth += 1;
                score += depth;
            }
            '}' if !garbageing => {
                depth -= 1;
            }
            _ if garbageing => {
                garbage += 1;
            }
            '<' => { garbageing = true },
            _ => ()
        }
    }
    dbg!(score);
    dbg!(garbage);
    Ok(())
}
