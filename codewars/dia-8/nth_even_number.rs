// https://www.codewars.com/kata/5933a1f8552bc2750a0000ed/train/rust

// Minha solução
fn nth_even(n: u32) -> u32 {
    n * 2 - 2
}

// Melhor solução
fn nth_even(n: u32) -> u32 {
    (n - 1) * 2
}