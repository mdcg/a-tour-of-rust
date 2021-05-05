// https://www.codewars.com/kata/5583090cbe83f4fd8c000051/train/rust

// Minha solução
fn digitize(n: u64) -> Vec<u8> {
    let mut v = Vec::<u8>::new();
    let n_str = n.to_string();
    for c in n_str.chars() {
        v.push(c.to_digit(10).unwrap() as u8);
    }
    v.reverse();
    return v;
}

// Melhor solução
fn digitize(n: u64) -> Vec<u8> {
    n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}