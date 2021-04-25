// https://www.codewars.com/kata/5ce9c1000bab0b001134f5af/train/rust

// Minha solução
fn quarter_of(month: u8) -> u8 {
    match month {
        1..=3 => 1,
        4..=6 => 2,
        7..=9 => 3,
        _ => 4
    }
}

// Melhor solução
fn quarter_of(m: u8) -> u8 {
    (m+2)/3
}