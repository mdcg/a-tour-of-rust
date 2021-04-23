// https://www.codewars.com/kata/55eca815d0d20962e1000106/train/rust

// Minha solução
fn generate_range(min: usize, max: usize, step: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut s = min;
    while s <= max {
        v.push(s);
        s += step;
    }
    return v;
}

// Melhor solução
fn generate_range(min: usize, max: usize, step: usize) -> Vec<usize> {
    (min..=max).step_by(step).collect()
}