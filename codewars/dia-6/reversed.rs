// https://www.codewars.com/kata/5a00e05cc374cb34d100000d/train/rust

// https://doc.rust-lang.org/std/vec/struct.Vec.html
// https://doc.rust-lang.org/std/primitive.usize.html

// Minha solução
fn reverse_seq(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    let mut count: usize = 0;
    let mut curr: u32 = n;
    while count < n as usize {
        vec.push(curr);
        curr -= 1;
        count += 1;
    }
    return vec;
}

// Melhor solução
fn reverse_seq(n: u32) -> Vec<u32> {
    (1..n + 1).rev().collect()
}