// https://www.codewars.com/kata/5875b200d520904a04000003/train/rust

// Minha solução
fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let fit = cap - on - wait;
    if fit < 0i32 {
        return fit.abs()
    } else {    
        return 0i32;
    }
}

// Melhor solução
fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    (on + wait - cap).max(0)
}

// Solução interessante
fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    match on + wait < cap {
        true =>  0,
        false => on + wait - cap,
    }
}