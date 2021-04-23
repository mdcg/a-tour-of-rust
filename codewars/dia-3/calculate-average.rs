// https://www.codewars.com/kata/57a2013acf1fa5bfc4000921/train/rust

// Minha solução
fn find_average(slice: &[f64]) -> f64 {
    if slice.len() == 0 {
        return 0f64;
    }

    let mut sum: f64 = slice.iter().sum();
    return sum / slice.len() as f64;
}

// Melhor solução
fn find_average(xs: &[f64]) -> f64 {
    match xs.len() {
        0 => 0.,
        n => xs.iter().sum::<f64>() / n as f64
    }
}