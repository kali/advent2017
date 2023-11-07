fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.to_string();
    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (mut x, mut y) = (maze[0].iter().position(|c| *c == '|').unwrap(), 0usize);
    let mut dir = (0isize, 1isize);
    let mut letters = String::new();
    let mut steps = 0;

    loop {
        match maze[y][x] {
            '+' if dir.0 == 0 => dir = (if maze[y][x - 1] != ' ' { -1 } else { 1 }, 0),
            '+' => dir = (0, if maze[y - 1][x] != ' ' { -1 } else { 1 }),
            ' ' => break,
            'A'..='Z' => letters.push(maze[y][x]),
            _ => (),
        }

        (x, y) = ((x as isize + dir.0) as usize, (y as isize + dir.1) as usize);
        steps += 1;
    }

    dbg!(letters);
    dbg!(steps);
    Ok(())
}
