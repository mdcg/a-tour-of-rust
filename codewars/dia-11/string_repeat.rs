// https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/rust

// Minha solução
fn repeat_str(src: &str, count: usize) -> String {
    let mut v = Vec::<&str>::new();
    for _ in 0..count {
        v.push(src);
    }
    return v.concat();
}

// Melhor solução
fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}