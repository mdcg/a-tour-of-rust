// https://www.codewars.com/kata/51c8991dee245d7ddf00000e/train/rust

// Minha solução
fn reverse_words(str: &str) -> String {
    let mut r: Vec<&str> = str.split(" ").collect();
    r.reverse();
    return r.join(" ");
}

// Melhor solução
fn reverse_words(str: &str) -> String {
    return str.split_whitespace()
         .rev()
         .collect::<Vec<_>>()
         .join(" ");
 }