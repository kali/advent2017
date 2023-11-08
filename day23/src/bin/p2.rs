fn is_prime(i: usize) -> bool {
    let sqrt = (i as f64).sqrt().ceil() as usize;
    !(2..sqrt).any(|d| i / d * d == i)
}

fn main() {
    let mut b = 108400;
    let mut count = 0;
    while b <= 125400 {
        if !is_prime(b) {
            count += 1;
        }
        b += 17;
    }
    dbg!(count);
}
