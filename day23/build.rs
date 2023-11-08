use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("func.c");
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "long func(long mode) {{").unwrap();
    writeln!(f, "long a = mode; long mul = 0;").unwrap();
    for c in 'b'..='h' {
        writeln!(f, "long {c} = 0;").unwrap();
    }
    let lines: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    for (ix, line) in lines.iter().enumerate() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        writeln!(f, "l{ix:02}: ").unwrap();
        match tokens[0] {
            "set" => writeln!(f, "{} = {};", tokens[1], tokens[2]).unwrap(),
            "mul" => writeln!(f, "{} *= {}; mul += 1;", tokens[1], tokens[2]).unwrap(),
            "sub" => writeln!(f, "{} -= {};", tokens[1], tokens[2]).unwrap(),
            "jnz" => writeln!(
                f,
                "if({} != 0) goto l{:02};",
                tokens[1],
                ix as isize + tokens[2].parse::<isize>().unwrap()
            )
            .unwrap(),
            _ => panic!("{line}"),
        }
    }
    writeln!(f, "l{:02}:", lines.len()).unwrap();
    writeln!(f, "return mode == 1 ? h : mul;").unwrap();
    writeln!(f, "}}").unwrap();
    std::mem::drop(f);

    cc::Build::new().file(dest_path).compile("foo");
}
